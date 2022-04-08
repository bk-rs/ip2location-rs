use std::{io::SeekFrom, net::Ipv4Addr};

use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt as _, AsyncSeekExt as _},
};

use crate::{
    bin_format::{
        database::LookupError,
        header::{Header, Type},
        ipv4_index::Ipv4Index,
        record::parse as record_parse,
    },
    record::Record,
};

//
#[derive(Debug)]
pub struct Ipv4Data {
    file: TokioFile,
    index: Ipv4Index,
    info: Ipv4DataInfo,
    r#type: Type,
    buf: Vec<u8>,
}

impl Ipv4Data {
    pub fn new(file: TokioFile, index: Ipv4Index, header: Header) -> Self {
        Self {
            file,
            index,
            info: header.ipv4_data_info,
            r#type: header.r#type,
            buf: {
                // 4 = ip_to(Ipv4Addr) size
                let len = Ipv4Data::len(1, header.num_fields) as usize + 4;
                let mut buf = Vec::with_capacity(len);
                buf.resize_with(len, Default::default);
                buf
            },
        }
    }

    pub fn len(n: u32, num_fields: u8) -> u32 {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L201
        n * (num_fields as u32) * 4
    }

    pub async fn lookup(&mut self, ip: Ipv4Addr) -> Result<Option<Record>, LookupError> {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L194-L210

        let (mut low, mut high) = self.index.low_and_high(ip);

        let fields = self.r#type.fields();
        let num_fields = fields.len();

        while low <= high {
            let mid = (low + high) >> 1;

            let offset = self.info.index_start + Ipv4Data::len(mid, num_fields as u8);

            self.file
                .seek(SeekFrom::Start(offset as u64 - 1))
                .await
                .map_err(LookupError::FileSeekFailed)?;

            self.file
                .read_exact(&mut self.buf)
                .await
                .map_err(LookupError::FileReadFailed)?;

            let ip_from = Ipv4Addr::from(u32::from_ne_bytes(self.buf[0..4].try_into().unwrap()));
            let ip_to = Ipv4Addr::from(u32::from_ne_bytes(
                self.buf[self.buf.len() - 4..self.buf.len()]
                    .try_into()
                    .unwrap(),
            ));

            #[allow(clippy::collapsible_else_if)]
            if (ip >= ip_from) && (ip < ip_to) {
                let record = record_parse(
                    ip_from.into(),
                    ip_to.into(),
                    &self.buf[4..self.buf.len() - 4],
                    &fields[1..],
                )
                .map_err(LookupError::RecordParseFailed)?;

                return Ok(Some(record));
            } else {
                if ip < ip_from {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }

        Ok(None)
    }
}

//
//
//
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct Ipv4DataInfo {
    // >= 0
    pub count: u32,
    // > 0
    pub index_start: u32,
}

impl Ipv4DataInfo {
    pub fn index_end(&self, num_fields: u8) -> u32 {
        self.index_start + Ipv4Data::len(self.count, num_fields)
    }
}
