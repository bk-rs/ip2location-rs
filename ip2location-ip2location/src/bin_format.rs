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

        if !inner.header.r#type.is_ip2location() {
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

        if !inner.header.r#type.is_ip2location() {
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

#[cfg(feature = "tokio_fs")]
#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, io::ErrorKind as IoErrorKind};

    #[tokio::test]
    async fn test_new_and_lookup_20220329() -> Result<(), Box<dyn error::Error>> {
        let path_bin_v4 = "data/ip2location-lite/20220329/IP2LOCATION-LITE-DB11.BIN";
        let path_bin_v6 = "data/ip2location-lite/20220329/IP2LOCATION-LITE-DB11.IPV6.BIN";

        let mut db_v4 = match Database::<TokioFile>::new(path_bin_v4).await {
            Ok(x) => Some(x),
            Err(DatabaseNewError::QuerierNewError(QuerierNewError::OpenFailed(err)))
                if err.kind() == IoErrorKind::NotFound =>
            {
                None
            }
            Err(err) => panic!("{:?}", err),
        };
        let mut db_v6 = match Database::<TokioFile>::new(path_bin_v6).await {
            Ok(x) => Some(x),
            Err(DatabaseNewError::QuerierNewError(QuerierNewError::OpenFailed(err)))
                if err.kind() == IoErrorKind::NotFound =>
            {
                None
            }
            Err(err) => panic!("{:?}", err),
        };

        if let Some(db_v4) = db_v4.as_mut() {
            let record_1 = db_v4
                .lookup(Ipv4Addr::from(16777216).into(), None)
                .await?
                .unwrap();
            assert_eq!(record_1.country_code.to_string(), "US");
            assert_eq!(record_1.latitude.unwrap(), 34.052_23);

            let selected_fields = &[
                RecordField::CountryCodeAndName,
                RecordField::RegionName,
                RecordField::Latitude,
            ];
            let record_2 = db_v4
                .lookup(Ipv4Addr::from(16777472).into(), selected_fields.as_ref())
                .await?
                .unwrap();
            assert_eq!(record_2.country_code.to_string(), "CN");
            println!("{:?}", record_2);
        }

        if let Some(db_v6) = db_v6.as_mut() {
            let record_1 = db_v6
                .lookup(Ipv6Addr::from(281470698520576).into(), None)
                .await?
                .unwrap();
            assert_eq!(record_1.country_code.to_string(), "US");

            let selected_fields = &[
                RecordField::CountryCodeAndName,
                RecordField::RegionName,
                RecordField::Latitude,
            ];
            let record_2 = db_v6
                .lookup(
                    Ipv6Addr::from(281470698520832).into(),
                    selected_fields.as_ref(),
                )
                .await?
                .unwrap();
            assert_eq!(record_2.country_code.to_string(), "CN");
            println!("{:?}", record_2);
        }

        Ok(())
    }
}
