use std::{io::SeekFrom, net::Ipv6Addr};

use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt as _, AsyncSeekExt as _},
};

use crate::{
    bin_format::{
        database::LookupError,
        header::{Header, Type},
        ipv6_index::Ipv6Index,
        record::parse as record_parse,
    },
    record::Record,
};

//
#[derive(Debug)]
pub struct Ipv6Data {
    file: TokioFile,
    index: Ipv6Index,
    info: Ipv6DataInfo,
    r#type: Type,
    buf: Vec<u8>,
}

impl Ipv6Data {
    pub fn new(file: TokioFile, index: Ipv6Index, header: Header) -> Self {
        Self {
            file,
            index,
            info: header.ipv6_data_info,
            r#type: header.r#type,
            buf: {
                // 16 = ip_to(Ipv6Addr) size
                let len = Ipv6Data::len(1, header.num_fields) as usize + 16;
                let mut buf = Vec::with_capacity(len);
                buf.resize_with(len, Default::default);
                buf
            },
        }
    }

    pub fn len(n: u32, num_fields: u8) -> u32 {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L231
        // 16 = Ipv6Addr size
        // 12 = 16 - 4
        n * ((num_fields as u32) * 4 + 12)
    }

    pub async fn lookup(&mut self, ip: Ipv6Addr) -> Result<Option<Record>, LookupError> {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L222-L241

        let (mut low, mut high) = self.index.low_and_high(ip);

        let fields = self.r#type.fields();
        let num_fields = fields.len();

        while low <= high {
            let mid = (low + high) >> 1;

            let offset = self.info.index_start + Ipv6Data::len(mid, num_fields as u8);

            self.file
                .seek(SeekFrom::Start(offset as u64 - 1))
                .await
                .map_err(LookupError::FileSeekFailed)?;

            self.file
                .read_exact(&mut self.buf)
                .await
                .map_err(LookupError::FileReadFailed)?;

            let ip_from_array: [u8; 16] = self.buf[0..16].try_into().unwrap();
            let ip_from = Ipv6Addr::from(ip_from_array);
            let ip_to_array: [u8; 16] = self.buf[self.buf.len() - 16..self.buf.len()]
                .try_into()
                .unwrap();
            let ip_to = Ipv6Addr::from(ip_to_array);

            #[allow(clippy::collapsible_else_if)]
            if (ip >= ip_from) && (ip < ip_to) {
                let record = record_parse(
                    ip_from.into(),
                    ip_to.into(),
                    &self.buf[16..self.buf.len() - 16],
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
pub struct Ipv6DataInfo {
    // >= 0
    pub count: u32,
    // > 0
    pub index_start: u32,
}

impl Ipv6DataInfo {
    pub fn index_end(&self, num_fields: u8) -> u32 {
        self.index_start + Ipv6Data::len(self.count, num_fields)
    }
}
