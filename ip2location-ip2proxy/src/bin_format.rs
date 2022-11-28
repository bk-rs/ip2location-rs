//
#[cfg(feature = "tokio_fs")]
pub type TokioFile = async_compat::Compat<tokio::fs::File>;

#[cfg(feature = "async_fs")]
pub type AsyncFsFile = async_fs::File;

use core::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use futures_util::{AsyncRead, AsyncSeek};
use ip2location_bin_format::querier::{
    LookupError as QuerierLookupError, NewError as QuerierNewError, Querier,
};

use crate::record::{OptionRecord, Record, RecordField};

//
pub struct Database<S> {
    pub inner: Querier<S>,
}

impl<S> fmt::Debug for Database<S>
where
    Querier<S>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Database")
            .field("inner", &self.inner)
            .finish()
    }
}

#[cfg(feature = "tokio_fs")]
impl Database<async_compat::Compat<tokio::fs::File>> {
    pub async fn new(
        path: impl AsRef<std::path::Path>,
        pool_max_size: usize,
    ) -> Result<Self, DatabaseNewError> {
        use futures_util::TryFutureExt as _;

        let path = path.as_ref().to_owned();

        let inner = Querier::new(
            || Box::pin(tokio::fs::File::open(path.clone()).map_ok(async_compat::Compat::new)),
            pool_max_size,
        )
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
    pub async fn new(
        path: impl AsRef<std::path::Path>,
        pool_max_size: usize,
    ) -> Result<Self, DatabaseNewError> {
        let path = path.as_ref().to_owned();

        let inner = Querier::new(
            || Box::pin(async_fs::File::open(path.clone())),
            pool_max_size,
        )
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
        &self,
        ip: IpAddr,
        selected_fields: impl Into<Option<&[RecordField]>>,
    ) -> Result<Option<Record>, DatabaseLookupError> {
        match ip {
            IpAddr::V4(ip) => self.lookup_ipv4(ip, selected_fields).await,
            IpAddr::V6(ip) => self.lookup_ipv6(ip, selected_fields).await,
        }
    }

    pub async fn lookup_ipv4(
        &self,
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
            Some(x) => Ok(OptionRecord::try_from(x)
                .map_err(DatabaseLookupError::ToRecordFailed)?
                .0),
            None => Ok(None),
        }
    }

    pub async fn lookup_ipv6(
        &self,
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
            Some(x) => Ok(OptionRecord::try_from(x)
                .map_err(DatabaseLookupError::ToRecordFailed)?
                .0),
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

#[cfg(feature = "tokio_fs")]
#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, io::ErrorKind as IoErrorKind};

    #[tokio::test]
    async fn test_new_and_lookup_20221101() -> Result<(), Box<dyn error::Error>> {
        let path_bin = "data/ip2proxy-lite/20221101/IP2PROXY-LITE-PX11.BIN";

        let db = match Database::<TokioFile>::new(path_bin, 1).await {
            Ok(x) => Some(x),
            Err(DatabaseNewError::QuerierNewError(QuerierNewError::OpenFailed(err)))
                if err.kind() == IoErrorKind::NotFound =>
            {
                None
            }
            Err(err) => panic!("{:?}", err),
        };

        if let Some(db) = db {
            let record_1 = db
                .lookup(Ipv4Addr::from(16778241).into(), None)
                .await?
                .unwrap();
            assert_eq!(record_1.country_code.to_string(), "AU");

            let selected_fields = &[RecordField::CountryCodeAndName, RecordField::RegionName];
            let record_2 = db
                .lookup(
                    Ipv6Addr::from(281470698521601).into(),
                    selected_fields.as_ref(),
                )
                .await?
                .unwrap();
            assert_eq!(record_2.country_code.to_string(), "AU");
            println!("{:?}", record_2);

            let record_3 = db
                .lookup(
                    Ipv6Addr::from(58569071808060804026606586837353981081).into(),
                    None,
                )
                .await?
                .unwrap();
            assert_eq!(record_3.country_code.to_string(), "RW");

            //
            let ret = db.lookup(Ipv4Addr::new(8, 8, 8, 8).into(), None).await?;
            assert!(ret.is_none());

            // google.com
            let ret = db
                .lookup(
                    "2607:f8b0:4009:817::200e"
                        .parse::<Ipv6Addr>()
                        .unwrap()
                        .into(),
                    None,
                )
                .await?;
            assert!(ret.is_none());
        }

        Ok(())
    }
}
