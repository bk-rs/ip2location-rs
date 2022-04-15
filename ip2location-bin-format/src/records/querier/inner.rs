use std::{
    io::SeekFrom,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use futures_util::{AsyncRead, AsyncReadExt as _, AsyncSeek, AsyncSeekExt as _};

use super::error::Error;
use crate::{
    record_field::{RecordFieldContent, RecordFieldContents, RecordFields},
    records::PositionRange,
};

//
#[derive(Debug)]
pub(super) struct Inner<S> {
    stream: S,
    count: u32,
    seek_from_start_base: u64,
    record_fields: RecordFields,
    record_field_contents: RecordFieldContents,
    buf: Vec<u8>,
}

impl<S> Inner<S> {
    pub(super) fn new(
        stream: S,
        count: u32,
        seek_from_start_base: u64,
        record_fields: RecordFields,
        record_field_contents: RecordFieldContents,
        buf: Vec<u8>,
    ) -> Self {
        Self {
            stream,
            count,
            seek_from_start_base,
            record_fields,
            record_field_contents,
            buf,
        }
    }
}

//
//
//
impl<S> Inner<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub(super) async fn query(
        &mut self,
        ip: IpAddr,
        PositionRange {
            start: mut low,
            end: mut high,
        }: PositionRange,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, Error> {
        if high > self.count {
            high = self.count;
        }
        if low > high {
            low = high;
        }

        let mut n_depth = 0;
        while low <= high {
            let mid = (low + high) >> 1;

            let seek_from_start = self.seek_from_start_base
                + match ip {
                    IpAddr::V4(_) => self.record_fields.records_bytes_len_for_ipv4(mid),
                    IpAddr::V6(_) => self.record_fields.records_bytes_len_for_ipv6(mid),
                } as u64;

            self.stream
                .seek(SeekFrom::Start(seek_from_start))
                .await
                .map_err(Error::SeekFailed)?;

            self.stream
                .read_exact(&mut self.buf)
                .await
                .map_err(Error::ReadFailed)?;

            let ip_from: IpAddr = match ip {
                IpAddr::V4(_) => {
                    Ipv4Addr::from(u32::from_ne_bytes(self.buf[0..4].try_into().unwrap())).into()
                }
                IpAddr::V6(_) => {
                    Ipv6Addr::from(u128::from_ne_bytes(self.buf[0..16].try_into().unwrap())).into()
                }
            };
            let ip_to: IpAddr = if high < self.count {
                match ip {
                    IpAddr::V4(_) => Ipv4Addr::from(u32::from_ne_bytes(
                        self.buf[self.buf.len() - 4..self.buf.len()]
                            .try_into()
                            .unwrap(),
                    ))
                    .into(),
                    IpAddr::V6(_) => Ipv6Addr::from(u128::from_ne_bytes(
                        self.buf[self.buf.len() - 16..self.buf.len()]
                            .try_into()
                            .unwrap(),
                    ))
                    .into(),
                }
            } else {
                match ip_from {
                    IpAddr::V4(ip_from) => {
                        Ipv4Addr::from(u32::from(ip_from).saturating_add(1)).into()
                    }
                    IpAddr::V6(ip_from) => {
                        Ipv6Addr::from(u128::from(ip_from).saturating_add(1)).into()
                    }
                }
            };

            if (ip >= ip_from) && (ip < ip_to) {
                let mut record_field_contents = self.record_field_contents.to_owned();
                for (n, record_field_content) in record_field_contents.iter_mut().enumerate() {
                    let index = match ip {
                        IpAddr::V4(_) => 4 + n as usize * 4,
                        IpAddr::V6(_) => 16 + n as usize * 4,
                    };

                    let content_index =
                        u32::from_ne_bytes(self.buf[index..index + 4].try_into().unwrap());

                    match record_field_content {
                        RecordFieldContent::COUNTRY(i, _, _) => *i = content_index,
                        RecordFieldContent::REGION(i, _) => *i = content_index,
                        RecordFieldContent::CITY(i, _) => *i = content_index,
                        RecordFieldContent::LATITUDE(v) => {
                            *v = {
                                f32::from_ne_bytes(self.buf[index..index + 4].try_into().unwrap())
                            }
                        }
                        RecordFieldContent::LONGITUDE(v) => {
                            *v = {
                                f32::from_ne_bytes(self.buf[index..index + 4].try_into().unwrap())
                            }
                        }
                        RecordFieldContent::ZIPCODE(i, _) => *i = content_index,
                        RecordFieldContent::TIMEZONE(i, _) => *i = content_index,
                        RecordFieldContent::PROXYTYPE(i, _) => *i = content_index,
                        RecordFieldContent::ISP(i, _) => *i = content_index,
                        RecordFieldContent::DOMAIN(i, _) => *i = content_index,
                        RecordFieldContent::USAGETYPE(i, _) => *i = content_index,
                        RecordFieldContent::ASN(i, _) => *i = content_index,
                        RecordFieldContent::AS(i, _) => *i = content_index,
                        RecordFieldContent::LASTSEEN(i, _) => *i = content_index,
                        RecordFieldContent::THREAT(i, _) => *i = content_index,
                        RecordFieldContent::RESIDENTIAL(i, _) => *i = content_index,
                        RecordFieldContent::PROVIDER(i, _) => *i = content_index,
                    }
                }

                return Ok(Some((ip_from, ip_to, record_field_contents)));
            } else if ip < ip_from {
                high = mid.saturating_sub(1);
            } else {
                low = mid.saturating_add(1);
            }

            //
            //
            //
            if high == 0 {
                return Ok(None);
            }
            #[allow(clippy::collapsible_else_if)]
            if self.count == u32::MAX {
                if low == self.count {
                    return Ok(None);
                }
            } else {
                if low > self.count {
                    return Ok(None);
                }
            }

            if n_depth > 30 {
                return Err(Error::MaxDepthReached);
            }

            n_depth += 1;
        }

        Ok(None)
    }
}
