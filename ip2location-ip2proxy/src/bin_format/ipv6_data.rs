use std::{io::SeekFrom, net::Ipv6Addr};

use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt as _, AsyncSeekExt as _},
};

use crate::bin_format::{database::LookupError, header::Header, ipv6_index::Ipv6Index};

//
#[derive(Debug)]
pub struct Ipv6Data {
    file: TokioFile,
    offset_base: u32,
    num_fields: u8,
    buf: Vec<u8>,
}

impl Ipv6Data {
    pub fn new(file: TokioFile, header: Header) -> Self {
        Self {
            file,
            offset_base: header.ipv6_data_info.index_start,
            num_fields: header.num_fields,
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

    pub async fn lookup(
        &mut self,
        ip: Ipv6Addr,
        ip_index: &Ipv6Index,
    ) -> Result<Option<(Ipv6Addr, Ipv6Addr, Vec<u32>)>, LookupError> {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L222-L241

        let (mut low, mut high) = ip_index.low_and_high(ip);

        while low <= high {
            let mid = (low + high) >> 1;

            let offset = self.offset_base + Ipv6Data::len(mid, self.num_fields);

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
                let mut indexes = vec![];
                for n in 1..self.num_fields as usize {
                    let i = n - 1;
                    let index = 16 + i * 4;

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
