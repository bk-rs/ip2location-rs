use core::fmt;
use std::net::{IpAddr, Ipv4Addr};

use futures_util::{AsyncRead, AsyncSeek};

use super::{error::Error, inner::Inner};
use crate::{header::schema::Schema, record_field::RecordFieldContents, records::PositionRange};

//
#[derive(Debug)]
pub struct V4Querier<S> {
    inner: Inner<S>,
}

//
//
//
impl<S> V4Querier<S> {
    pub fn new(stream: S, header: Schema) -> Result<Self, NewError> {
        let record_fields = header
            .record_fields()
            .ok_or(NewError::RecordFieldsMissing)?;

        let record_field_contents = record_fields.to_contents();

        let buf = {
            let len = record_fields.record_bytes_len_for_ipv4_with_double_ip() as usize;
            let mut buf = Vec::with_capacity(len);
            buf.resize_with(len, Default::default);
            buf
        };

        Ok(Self {
            inner: Inner::new(
                stream,
                header.v4_records_count,
                header.v4_records_seek_from_start(),
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
}

impl fmt::Display for NewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for NewError {}

//
//
//
impl<S> V4Querier<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub async fn query(
        &mut self,
        ip: Ipv4Addr,
        position_range: PositionRange,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, Error> {
        self.inner.query(ip.into(), position_range).await
    }
}
