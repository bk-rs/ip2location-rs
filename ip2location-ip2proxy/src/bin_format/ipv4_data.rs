use std::{io::SeekFrom, net::Ipv4Addr};

use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt as _, AsyncSeekExt as _},
};

use crate::bin_format::{database::LookupError, header::Header, ipv4_index::Ipv4Index};

//
#[derive(Debug)]
pub struct Ipv4Data {
    file: TokioFile,
    offset_base: u32,
    num_fields: u8,
    buf: Vec<u8>,
}

impl Ipv4Data {
    pub fn new(file: TokioFile, header: Header) -> Self {
        Self {
            file,
            offset_base: header.ipv4_data_info.index_start,
            num_fields: header.num_fields,
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

    pub async fn lookup(
        &mut self,
        ip: Ipv4Addr,
        ip_index: &Ipv4Index,
    ) -> Result<Option<(Ipv4Addr, Ipv4Addr, Vec<u32>)>, LookupError> {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L194-L210

        let (mut low, mut high) = ip_index.low_and_high(ip);

        while low <= high {
            let mid = (low + high) >> 1;

            let offset = self.offset_base + Ipv4Data::len(mid, self.num_fields);

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
                let mut indexes = vec![];
                for n in 1..self.num_fields as usize {
                    let i = n - 1;
                    let index = 4 + i * 4;

                    indexes.push(u32::from_ne_bytes(
                        self.buf[index..index + 4].try_into().unwrap(),
                    ))
                }

                return Ok(Some((ip_from, ip_to, indexes)));
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
