use core::fmt;
use std::net::{IpAddr, Ipv6Addr};

use futures_util::{AsyncRead, AsyncSeek};

use super::{error::Error, inner::Inner};
use crate::{header::schema::Schema, record_field::RecordFieldContents, records::PositionRange};

//
#[derive(Debug)]
pub struct V6Querier<S> {
    inner: Inner<S>,
}

//
//
//
impl<S> V6Querier<S> {
    pub fn new(stream: S, header: Schema) -> Result<Self, NewError> {
        let record_fields = header
            .record_fields()
            .ok_or(NewError::RecordFieldsMissing)?;

        let record_field_contents = record_fields.to_contents();

        let buf = {
            let len = if header.has_v6() {
                record_fields.record_bytes_len_for_ipv6_with_double_ip() as usize
            } else {
                return Err(NewError::Unsupported);
            };
            let mut buf = Vec::with_capacity(len);
            buf.resize_with(len, Default::default);
            buf
        };

        Ok(Self {
            inner: Inner::new(
                stream,
                header.v6_records_count,
                header
                    .v6_records_seek_from_start()
                    .ok_or(NewError::Unsupported)?,
                record_fields,
                record_field_contents,
                buf,
            ),
        })
    }
}

//
#[derive(Debug)]
pub enum NewError {
    RecordFieldsMissing,
    Unsupported,
}

impl fmt::Display for NewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for NewError {}

//
//
//
impl<S> V6Querier<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub async fn query(
        &mut self,
        ip: Ipv6Addr,
        position_range: PositionRange,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, Error> {
        self.inner.query(ip.into(), position_range).await
    }
}
