use core::{fmt, future::Future, ops::ControlFlow, pin::Pin};
use std::{
    io::{Cursor, Error as IoError, SeekFrom},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

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
#[derive(Debug)]
pub struct Querier<S> {
    pub header: HeaderSchema,
    pub index_v4: IndexV4Querier,
    pub index_v6: Option<IndexV6Querier>,
    pub records_v4: RecordsV4Querier<S>,
    pub records_v6: Option<RecordsV6Querier<S>>,
    pub content: ContentQuerier<S>,
}

//
//
//
impl<S> Querier<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub async fn new<F>(mut stream_repeater: F) -> Result<Self, NewError>
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

        let records_v4 = {
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
            RecordsV4Querier::new(stream, header).map_err(NewError::RecordsV4QuerierNewFailed)?
        };

        let mut records_v6 = None;
        #[allow(clippy::unnecessary_operation)]
        {
            if header.has_v6() {
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
                let records_v6_tmp = RecordsV6Querier::new(stream, header)
                    .map_err(NewError::RecordsV6QuerierNewFailed)?;

                records_v6 = Some(records_v6_tmp);
            }
        };

        let content = {
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
            ContentQuerier::new(stream)
        };

        //
        Ok(Self {
            header,
            index_v4,
            index_v6,
            records_v4,
            records_v6,
            content,
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

impl fmt::Display for NewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
        &mut self,
        ip: IpAddr,
        selected_fields: impl Into<Option<&[RecordField]>>,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, LookupError> {
        match ip {
            IpAddr::V4(ip) => self.lookup_ipv4(ip, selected_fields).await,
            IpAddr::V6(ip) => self.lookup_ipv6(ip, selected_fields).await,
        }
    }

    pub async fn lookup_ipv4(
        &mut self,
        ip: Ipv4Addr,
        selected_fields: impl Into<Option<&[RecordField]>>,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, LookupError> {
        let position_range = self.index_v4.query(ip);

        if position_range.end == 0 {
            return Ok(None);
        }

        let (ip_from, ip_to, mut record_field_contents) = match self
            .records_v4
            .query(ip, position_range)
            .await
            .map_err(LookupError::RecordsQueryFailed)?
        {
            Some(x) => x,
            None => return Ok(None),
        };

        if let Some(selected_fields) = selected_fields.into() {
            record_field_contents.select(selected_fields);
        }

        self.content
            .fill(&mut record_field_contents)
            .await
            .map_err(LookupError::ContentFillFailed)?;

        Ok(Some((ip_from, ip_to, record_field_contents)))
    }

    pub async fn lookup_ipv6(
        &mut self,
        ip: Ipv6Addr,
        selected_fields: impl Into<Option<&[RecordField]>>,
    ) -> Result<Option<(IpAddr, IpAddr, RecordFieldContents)>, LookupError> {
        if let Some(ip) = ip.to_ipv4() {
            return self.lookup_ipv4(ip, selected_fields).await;
        }

        let position_range = self
            .index_v6
            .as_ref()
            .map(|x| x.query(ip))
            .unwrap_or_default();

        if position_range.end == 0 {
            return Ok(None);
        }

        let (ip_from, ip_to, mut record_field_contents) = match self.records_v6.as_mut() {
            Some(records_v6) => {
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

        if let Some(selected_fields) = selected_fields.into() {
            record_field_contents.select(selected_fields);
        }

        self.content
            .fill(&mut record_field_contents)
            .await
            .map_err(LookupError::ContentFillFailed)?;

        Ok(Some((ip_from, ip_to, record_field_contents)))
    }
}

//
#[derive(Debug)]
pub enum LookupError {
    RecordsQueryFailed(RecordsQueryError),
    ContentFillFailed(ContentFillError),
}

impl fmt::Display for LookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LookupError {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use async_compat::Compat;
    use futures_util::TryFutureExt as _;
    use tokio::fs::File as TokioFile;

    use crate::test_helper::{ip2location_bin_files, ip2proxy_bin_files};

    #[tokio::test]
    async fn test_new_and_lookup() -> Result<(), Box<dyn error::Error>> {
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
            let mut q =
                Querier::new(|| Box::pin(TokioFile::open(path.clone()).map_ok(Compat::new)))
                    .await?;

            for ip in ips {
                let ret = q.lookup(*ip, None).await?;
                assert!(ret.is_none());
            }
        }

        for path in ip2proxy_bin_files().iter() {
            let mut q =
                Querier::new(|| Box::pin(TokioFile::open(path.clone()).map_ok(Compat::new)))
                    .await?;

            for ip in ips {
                let ret = q.lookup(*ip, None).await?;
                assert!(ret.is_none());
            }

            if path.as_os_str().to_str().map(|x| x.contains("/20220401")) == Some(true) {
                let ret = q
                    .lookup(
                        Ipv6Addr::from(58569071813452613185929873510317667680).into(),
                        None,
                    )
                    .await?;
                println!("{:?}", ret)
            }
        }

        Ok(())
    }
}
