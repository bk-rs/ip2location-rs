use core::{future::Future, ops::ControlFlow, pin::Pin};
use std::{
    io::{Cursor, Error as IoError, SeekFrom},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use deadpool::unmanaged::{Pool, PoolError};
use futures_util::{AsyncRead, AsyncReadExt as _, AsyncSeek, AsyncSeekExt as _};

use crate::{
    content::{querier::FillError as ContentFillError, Querier as ContentQuerier},
    header::{
        parser::ParseError as HeaderParseError, Parser as HeaderParser, Schema as HeaderSchema,
        HEADER_LEN,
    },
    index::{
        querier::BuildError as IndexBuildError, V4Querier as IndexV4Querier,
        V6Querier as IndexV6Querier, INDEX_LEN,
    },
    record_field::{RecordField, RecordFieldContents},
    records::{
        querier::v4_querier::NewError as RecordsV4QuerierNewError,
        querier::v6_querier::NewError as RecordsV6QuerierNewError,
        querier::Error as RecordsQueryError, V4Querier as RecordsV4Querier,
        V6Querier as RecordsV6Querier,
    },
};

//
pub struct Querier<S> {
    pub header: HeaderSchema,
    pub index_v4: IndexV4Querier,
    pub index_v6: Option<IndexV6Querier>,
    pub records_v4_pool: Pool<RecordsV4Querier<S>>,
    pub records_v6_pool: Option<Pool<RecordsV6Querier<S>>>,
    pub content_pool: Pool<ContentQuerier<S>>,
}

impl<S> core::fmt::Debug for Querier<S>
where
    Pool<RecordsV4Querier<S>>: core::fmt::Debug,
    Option<Pool<RecordsV6Querier<S>>>: core::fmt::Debug,
    Pool<ContentQuerier<S>>: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Querier")
            .field("header", &self.header)
            .field("index_v4", &self.index_v4)
            .field("index_v6", &self.index_v6)
            .field("records_v4_pool", &self.records_v4_pool)
            .field("records_v6_pool", &self.records_v6_pool)
            .field("content_pool", &self.content_pool)
            .finish()
    }
}

//
//
//
impl<S> Querier<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub async fn new<F>(mut stream_repeater: F, pool_max_size: usize) -> Result<Self, NewError>
    where
        F: FnMut() -> Pin<Box<dyn Future<Output = Result<S, IoError>> + Send + 'static>>,
    {
        let mut buf = vec![0; 1024 * 8];

        //
        let mut stream = stream_repeater().await.map_err(NewError::OpenFailed)?;

        //
        let header = {
            let mut parser = HeaderParser::new();
            let mut n_read = 0;
            let mut n_parsed = 0;
            loop {
                let n = stream
                    .read(&mut buf[n_read..n_read + HEADER_LEN as usize])
                    .await
                    .map_err(NewError::ReadFailed)?;

                if n == 0 {
                    return Err(NewError::ReadOtherError("header parsing is not completed"));
                }

                n_read += n;

                match parser
                    .parse(&mut Cursor::new(&buf[n_parsed..n_read]))
                    .map_err(NewError::HeaderParseFailed)?
                {
                    ControlFlow::Continue(n) => {
                        n_parsed += n;
                        continue;
                    }
                    ControlFlow::Break((_n, header)) => {
                        break header;
                    }
                }
            }
        };

        //
        stream
            .seek(SeekFrom::Start(header.total_size as u64))
            .await
            .map_err(NewError::SeekFailed)?;
        let n = stream
            .read(&mut buf[..1])
            .await
            .map_err(NewError::ReadFailed)?;
        if n != 0 {
            return Err(NewError::TotalSizeMissing);
        }

        //
        let index_v4 = {
            let mut builder = IndexV4Querier::builder();
            let mut n_max_appended = INDEX_LEN as usize;
            stream
                .seek(SeekFrom::Start(header.v4_index_seek_from_start()))
                .await
                .map_err(NewError::ReadFailed)?;
            loop {
                let n = stream
                    .read(&mut buf[..])
                    .await
                    .map_err(NewError::ReadFailed)?;

                if n == 0 {
                    return Err(NewError::ReadOtherError(
                        "index_v4 building is not completed",
                    ));
                }

                if n < n_max_appended {
                    builder.append(&buf[..n]);

                    n_max_appended -= n;
                    continue;
                } else {
                    builder.append(&buf[..n_max_appended]);

                    break builder
                        .finish::<IndexV4Querier>()
                        .map_err(NewError::IndexV4BuildFailed)?;
                }
            }
        };

        //
        let mut index_v6 = None;
        #[allow(clippy::unnecessary_operation)]
        {
            if let Some(v6_index_seek_from_start) = header.v6_index_seek_from_start() {
                let mut builder = IndexV6Querier::builder();
                let mut n_max_appended = INDEX_LEN as usize;
                stream
                    .seek(SeekFrom::Start(v6_index_seek_from_start))
                    .await
                    .map_err(NewError::ReadFailed)?;
                let index_v6_tmp = loop {
                    let n = stream
                        .read(&mut buf[..])
                        .await
                        .map_err(NewError::ReadFailed)?;

                    if n == 0 {
                        return Err(NewError::ReadOtherError(
                            "index_v6 building is not completed",
                        ));
                    }

                    if n < n_max_appended {
                        builder.append(&buf[..n]);

                        n_max_appended -= n;
                        continue;
                    } else {
                        builder.append(&buf[..n_max_appended]);

                        break builder
                            .finish::<IndexV6Querier>()
                            .map_err(NewError::IndexV6BuildFailed)?;
                    }
                };
                index_v6 = Some(index_v6_tmp);
            }
        };

        let records_v4_pool = {
            let mut pool_objs = vec![];

            for _ in 0..pool_max_size {
                let mut stream = stream_repeater().await.map_err(NewError::OpenFailed)?;

                stream
                    .seek(SeekFrom::Start(header.total_size as u64))
                    .await
                    .map_err(NewError::SeekFailed)?;
                let n = stream
                    .read(&mut buf[..1])
                    .await
                    .map_err(NewError::ReadFailed)?;
                if n != 0 {
                    return Err(NewError::TotalSizeMissing);
                }

                //
                let pool_obj = RecordsV4Querier::new(stream, header)
                    .map_err(NewError::RecordsV4QuerierNewFailed)?;

                pool_objs.push(pool_obj);
            }

            Pool::from(pool_objs)
        };

        let mut records_v6_pool = None;
        #[allow(clippy::unnecessary_operation)]
        {
            if header.has_v6() {
                let mut pool_objs = vec![];

                for _ in 0..pool_max_size {
                    let mut stream = stream_repeater().await.map_err(NewError::OpenFailed)?;

                    stream
                        .seek(SeekFrom::Start(header.total_size as u64))
                        .await
                        .map_err(NewError::SeekFailed)?;
                    let n = stream
                        .read(&mut buf[..1])
                        .await
                        .map_err(NewError::ReadFailed)?;
                    if n != 0 {
                        return Err(NewError::TotalSizeMissing);
                    }

                    //
                    let pool_obj = RecordsV6Querier::new(stream, header)
                        .map_err(NewError::RecordsV6QuerierNewFailed)?;

                    pool_objs.push(pool_obj);
                }

                records_v6_pool = Some(Pool::from(pool_objs))
            }
        };

        let content_pool = {
            let mut pool_objs = vec![];

            for _ in 0..pool_max_size {
                let mut stream = stream_repeater().await.map_err(NewError::OpenFailed)?;

                stream
                    .seek(SeekFrom::Start(header.total_size as u64))
                    .await
                    .map_err(NewError::SeekFailed)?;
                let n = stream
                    .read(&mut buf[..1])
                    .await
                    .map_err(NewError::ReadFailed)?;
                if n != 0 {
                    return Err(NewError::TotalSizeMissing);
                }

                //
                let pool_obj = ContentQuerier::new(stream);

                pool_objs.push(pool_obj);
            }

            Pool::from(pool_objs)
        };

        //
        Ok(Self {
            header,
            index_v4,
            index_v6,
            records_v4_pool,
            records_v6_pool,
            content_pool,
        })
    }
}

//
#[derive(Debug)]
pub enum NewError {
    OpenFailed(IoError),
    SeekFailed(IoError),
    ReadFailed(IoError),
    ReadOtherError(&'static str),
    HeaderParseFailed(HeaderParseError),
    TotalSizeMissing,
    IndexV4BuildFailed(IndexBuildError),
    IndexV6BuildFailed(IndexBuildError),
    RecordsV4QuerierNewFailed(RecordsV4QuerierNewError),
    RecordsV6QuerierNewFailed(RecordsV6QuerierNewError),
}

impl core::fmt::Display for NewError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for NewError {}

//
//
//
impl<S> Querier<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub async fn lookup(
        &self,
        ip: IpAddr,
        selected_fields: Option<&[RecordField]>,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, LookupError> {
        match ip {
            IpAddr::V4(ip) => self.lookup_ipv4(ip, selected_fields).await,
            IpAddr::V6(ip) => self.lookup_ipv6(ip, selected_fields).await,
        }
    }

    pub async fn lookup_ipv4(
        &self,
        ip: Ipv4Addr,
        selected_fields: Option<&[RecordField]>,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, LookupError> {
        let position_range = self.index_v4.query(ip);

        if position_range.end == 0 {
            return Ok(None);
        }

        //
        let mut records_v4 = self
            .records_v4_pool
            .get()
            .await
            .map_err(LookupError::PoolGetFailed)?;
        let (ip_from, ip_to, mut record_field_contents) = match records_v4
            .query(ip, position_range)
            .await
            .map_err(LookupError::RecordsQueryFailed)?
        {
            Some(x) => x,
            None => return Ok(None),
        };

        if let Some(selected_fields) = selected_fields {
            record_field_contents.select(selected_fields);
        }

        //
        let mut content = self
            .content_pool
            .get()
            .await
            .map_err(LookupError::PoolGetFailed)?;

        content
            .fill(&mut record_field_contents)
            .await
            .map_err(LookupError::ContentFillFailed)?;

        Ok(Some((ip_from, ip_to, record_field_contents)))
    }

    pub async fn lookup_ipv6(
        &self,
        ip: Ipv6Addr,
        selected_fields: Option<&[RecordField]>,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, LookupError> {
        if let Some(ip) = ip.to_ipv4() {
            return self.lookup_ipv4(ip, selected_fields).await.map(|x| {
                x.map(|(ip_from, ip_to, record_field_contents)| {
                    (
                        match ip_from {
                            IpAddr::V4(ip) => ip.to_ipv6_mapped().into(),
                            IpAddr::V6(ip) => {
                                debug_assert!(false, "unreachable");
                                ip.into()
                            }
                        },
                        match ip_to {
                            IpAddr::V4(ip) => ip.to_ipv6_mapped().into(),
                            IpAddr::V6(ip) => {
                                debug_assert!(false, "unreachable");
                                ip.into()
                            }
                        },
                        record_field_contents,
                    )
                })
            });
        }

        let position_range = self
            .index_v6
            .as_ref()
            .map(|x| x.query(ip))
            .unwrap_or_default();

        if position_range.end == 0 {
            return Ok(None);
        }

        let (ip_from, ip_to, mut record_field_contents) = match self.records_v6_pool.as_ref() {
            Some(records_v6_pool) => {
                //
                let mut records_v6 = records_v6_pool
                    .get()
                    .await
                    .map_err(LookupError::PoolGetFailed)?;

                match records_v6
                    .query(ip, position_range)
                    .await
                    .map_err(LookupError::RecordsQueryFailed)?
                {
                    Some(x) => x,
                    None => return Ok(None),
                }
            }
            None => return Ok(None),
        };

        if let Some(selected_fields) = selected_fields {
            record_field_contents.select(selected_fields);
        }

        //
        let mut content = self
            .content_pool
            .get()
            .await
            .map_err(LookupError::PoolGetFailed)?;

        content
            .fill(&mut record_field_contents)
            .await
            .map_err(LookupError::ContentFillFailed)?;

        Ok(Some((ip_from, ip_to, record_field_contents)))
    }
}

//
#[derive(Debug)]
pub enum LookupError {
    PoolGetFailed(PoolError),
    RecordsQueryFailed(RecordsQueryError),
    ContentFillFailed(ContentFillError),
}

impl core::fmt::Display for LookupError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LookupError {}

#[cfg(test)]
mod tests {
    use super::*;

    use async_compat::Compat;
    use futures_util::TryFutureExt as _;
    use tokio::fs::File as TokioFile;

    use crate::test_helper::{ip2location_bin_files, ip2proxy_bin_files};

    #[tokio::test]
    async fn test_new_and_lookup() -> Result<(), Box<dyn std::error::Error>> {
        let ips: &[IpAddr] = &[
            Ipv4Addr::new(0, 0, 0, 0).into(),
            Ipv4Addr::new(255, 255, 255, 255).into(),
            Ipv6Addr::new(0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0).into(),
            Ipv6Addr::new(
                0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
            )
            .into(),
        ];

        for path in ip2location_bin_files().iter() {
            let q = Querier::new(
                || Box::pin(TokioFile::open(path.clone()).map_ok(Compat::new)),
                2,
            )
            .await?;

            for ip in ips {
                let ret = q.lookup(*ip, None).await?;
                assert!(ret.is_none());
            }

            if path
                .as_os_str()
                .to_str()
                .map(|x| x.contains("/20220329") && x.contains("IPV6.BIN"))
                == Some(true)
            {
                q.lookup(
                    Ipv6Addr::from(58569107296622255421594597096899477504).into(),
                    None,
                )
                .await?
                .unwrap();
            }

            if path
                .as_os_str()
                .to_str()
                .map(|x| x.contains("/ip2location-sample") && x.contains("/IP-"))
                == Some(true)
            {
                let ret = q.lookup(Ipv4Addr::new(8, 8, 8, 8).into(), None).await?;
                assert!(ret.is_some());
            }
        }

        for path in ip2proxy_bin_files().iter() {
            let q = Querier::new(
                || Box::pin(TokioFile::open(path.clone()).map_ok(Compat::new)),
                2,
            )
            .await?;

            for ip in ips {
                let ret = q.lookup(*ip, None).await?;
                assert!(ret.is_none());
            }

            if path.as_os_str().to_str().map(|x| x.contains("/20221101")) == Some(true) {
                q.lookup(
                    Ipv6Addr::from(58569071813452613185929873510317667680).into(),
                    None,
                )
                .await?
                .unwrap();
            }
        }

        Ok(())
    }
}
