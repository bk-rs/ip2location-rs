use core::fmt;
use std::{
    io::{Error as IoError, SeekFrom},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use futures_util::{AsyncRead, AsyncReadExt as _, AsyncSeek, AsyncSeekExt as _};

use super::{Category, PositionRange};
use crate::{
    header::schema::Schema,
    record_field::{RecordField, RecordFields},
};

//
#[derive(Debug)]
pub struct Querier<S> {
    stream: S,
    category: Category,
    //
    seek_from_start_base: u64,
    record_fields: RecordFields,
    count: u32,
    buf: Vec<u8>,
}

//
//
//
impl<S> Querier<S> {
    pub fn new(stream: S, category: Category, header: Schema) -> Result<Self, BuildError> {
        let record_fields = header
            .record_fields()
            .ok_or(BuildError::RecordFieldsMissing)?;

        let buf = {
            let len = match category {
                Category::V4 => record_fields.record_bytes_len_for_ipv4_with_double_ip(),
                Category::V6 => {
                    if header.has_v6() {
                        record_fields.record_bytes_len_for_ipv6_with_double_ip()
                    } else {
                        return Err(BuildError::Unsupported);
                    }
                }
            } as usize;
            let mut buf = Vec::with_capacity(len);
            buf.resize_with(len, Default::default);
            buf
        };

        Ok(Self {
            stream,
            category,
            //
            seek_from_start_base: match category {
                Category::V4 => header.v4_records_seek_from_start(),
                Category::V6 => header
                    .v6_records_seek_from_start()
                    .ok_or(BuildError::Unsupported)?,
            },
            record_fields,
            count: match category {
                Category::V4 => header.v4_records_count,
                Category::V6 => header.v6_records_count,
            },
            buf,
        })
    }
}

//
#[derive(Debug)]
pub enum BuildError {
    RecordFieldsMissing,
    Unsupported,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BuildError {}

//
//
//
impl<S> Querier<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub async fn find(
        &mut self,
        ip: IpAddr,
        PositionRange {
            start: mut low,
            end: mut high,
        }: PositionRange,
    ) -> Result<Option<FindOutput>, FindError> {
        match ip {
            IpAddr::V4(_) => {
                if !matches!(self.category, Category::V4) {
                    return Err(FindError::Unsupported);
                }
            }
            IpAddr::V6(_) => {
                if !matches!(self.category, Category::V6) {
                    return Err(FindError::Unsupported);
                }
            }
        }

        while low <= high {
            let mid = (low + high) >> 1;

            let seek_from_start = self.seek_from_start_base
                + match self.category {
                    Category::V4 => self.record_fields.records_bytes_len_for_ipv4(mid),
                    Category::V6 => self.record_fields.records_bytes_len_for_ipv6(mid),
                } as u64;

            self.stream
                .seek(SeekFrom::Start(seek_from_start))
                .await
                .map_err(FindError::SeekFailed)?;

            self.stream
                .read_exact(&mut self.buf)
                .await
                .map_err(FindError::ReadFailed)?;

            let ip_from: IpAddr = match self.category {
                Category::V4 => {
                    Ipv4Addr::from(u32::from_ne_bytes(self.buf[0..4].try_into().unwrap())).into()
                }
                Category::V6 => {
                    let array: [u8; 16] = self.buf[0..16].try_into().unwrap();
                    Ipv6Addr::from(array).into()
                }
            };
            let ip_to: IpAddr = if high < self.count {
                match self.category {
                    Category::V4 => Ipv4Addr::from(u32::from_ne_bytes(
                        self.buf[self.buf.len() - 4..self.buf.len()]
                            .try_into()
                            .unwrap(),
                    ))
                    .into(),
                    Category::V6 => {
                        let array: [u8; 16] = self.buf[self.buf.len() - 16..self.buf.len()]
                            .try_into()
                            .unwrap();
                        Ipv6Addr::from(array).into()
                    }
                }
            } else {
                match ip_from {
                    IpAddr::V4(ip_from) => Ipv4Addr::from(u32::from(ip_from) + 1).into(),
                    IpAddr::V6(ip_from) => Ipv6Addr::from(u128::from(ip_from) + 1).into(),
                }
            };

            #[allow(clippy::collapsible_else_if)]
            if (ip >= ip_from) && (ip < ip_to) {
                let mut record_fields_with_index = vec![];
                for (i, (record_field, record_field_len)) in self
                    .record_fields
                    .field_and_len_list_without_ip()
                    .into_iter()
                    .enumerate()
                {
                    let index = match self.category {
                        Category::V4 => 4 + i as usize * record_field_len as usize,
                        Category::V6 => 16 + i as usize * record_field_len as usize,
                    };

                    record_fields_with_index.push((
                        *record_field,
                        u32::from_ne_bytes(
                            self.buf[index..index + record_field_len as usize]
                                .try_into()
                                .unwrap(),
                        ),
                    ))
                }

                return Ok(Some(FindOutput {
                    ip_from,
                    ip_to,
                    record_fields_with_index,
                }));
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
#[derive(Debug)]
pub struct FindOutput {
    pub ip_from: IpAddr,
    pub ip_to: IpAddr,
    pub record_fields_with_index: Vec<(RecordField, u32)>,
}

//
#[derive(Debug)]
pub enum FindError {
    Unsupported,
    SeekFailed(IoError),
    ReadFailed(IoError),
}

impl fmt::Display for FindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FindError {}
