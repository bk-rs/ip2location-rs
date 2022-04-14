use core::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use futures_util::{AsyncRead, AsyncSeek};
use ip2location_bin_format::querier::{
    LookupError as QuerierLookupError, NewError as QuerierNewError, Querier,
};

use crate::record::{Record, RecordField};

//
#[derive(Debug)]
pub struct Database<S> {
    pub inner: Querier<S>,
}

#[cfg(feature = "tokio_fs")]
impl Database<async_compat::Compat<tokio::fs::File>> {
    pub async fn new(path: impl AsRef<std::path::Path>) -> Result<Self, DatabaseNewError> {
        use futures_util::TryFutureExt as _;

        let path = path.as_ref().to_owned();

        let inner = Querier::new(|| {
            Box::pin(tokio::fs::File::open(path.clone()).map_ok(async_compat::Compat::new))
        })
        .await
        .map_err(DatabaseNewError::QuerierNewError)?;

        if !inner.header.r#type.is_ip2proxy() {
            return Err(DatabaseNewError::TypeMismatch);
        }

        Ok(Self { inner })
    }
}

#[cfg(feature = "async_fs")]
impl Database<async_fs::File> {
    pub async fn new(path: impl AsRef<std::path::Path>) -> Result<Self, DatabaseNewError> {
        let path = path.as_ref().to_owned();

        let inner = Querier::new(|| Box::pin(async_fs::File::open(path.clone())))
            .await
            .map_err(DatabaseNewError::QuerierNewError)?;

        if !inner.header.r#type.is_ip2proxy() {
            return Err(DatabaseNewError::TypeMismatch);
        }

        Ok(Self { inner })
    }
}

//
#[derive(Debug)]
pub enum DatabaseNewError {
    QuerierNewError(QuerierNewError),
    TypeMismatch,
}

impl fmt::Display for DatabaseNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DatabaseNewError {}

//
//
//
impl<S> Database<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub async fn lookup(
        &mut self,
        ip: IpAddr,
        selected_fields: impl Into<Option<&[RecordField]>>,
    ) -> Result<Option<Record>, DatabaseLookupError> {
        match ip {
            IpAddr::V4(ip) => self.lookup_ipv4(ip, selected_fields).await,
            IpAddr::V6(ip) => self.lookup_ipv6(ip, selected_fields).await,
        }
    }

    pub async fn lookup_ipv4(
        &mut self,
        ip: Ipv4Addr,
        selected_fields: impl Into<Option<&[RecordField]>>,
    ) -> Result<Option<Record>, DatabaseLookupError> {
        let selected_fields: Option<Vec<ip2location_bin_format::record_field::RecordField>> =
            selected_fields
                .into()
                .map(|x| x.iter().map(Into::into).collect::<Vec<_>>());
        let selected_fields = selected_fields.as_deref();

        //
        match self
            .inner
            .lookup_ipv4(ip, selected_fields)
            .await
            .map_err(DatabaseLookupError::QuerierLookupError)?
        {
            Some(x) => Ok(Some(
                Record::try_from(x).map_err(DatabaseLookupError::ToRecordFailed)?,
            )),
            None => Ok(None),
        }
    }

    pub async fn lookup_ipv6(
        &mut self,
        ip: Ipv6Addr,
        selected_fields: impl Into<Option<&[RecordField]>>,
    ) -> Result<Option<Record>, DatabaseLookupError> {
        let selected_fields: Option<Vec<ip2location_bin_format::record_field::RecordField>> =
            selected_fields
                .into()
                .map(|x| x.iter().map(Into::into).collect::<Vec<_>>());
        let selected_fields = selected_fields.as_deref();

        //
        match self
            .inner
            .lookup_ipv6(ip, selected_fields)
            .await
            .map_err(DatabaseLookupError::QuerierLookupError)?
        {
            Some(x) => Ok(Some(
                Record::try_from(x).map_err(DatabaseLookupError::ToRecordFailed)?,
            )),
            None => Ok(None),
        }
    }
}

//
#[derive(Debug)]
pub enum DatabaseLookupError {
    QuerierLookupError(QuerierLookupError),
    ToRecordFailed(Box<str>),
}

impl fmt::Display for DatabaseLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DatabaseLookupError {}
