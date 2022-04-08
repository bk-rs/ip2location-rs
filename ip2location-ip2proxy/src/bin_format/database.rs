use core::{fmt, ops::ControlFlow};
use std::{
    io::{Cursor, Error as IoError, SeekFrom},
    net::{Ipv4Addr, Ipv6Addr},
    path::Path,
};

use async_fs::File as AsyncFsFile;
use futures_util::{AsyncReadExt as _, AsyncSeekExt as _};
use tokio::fs::File as TokioFile;

use crate::{
    bin_format::{
        header::{
            Header, ParseError as HeaderParseError, Parser as HeaderParser,
            MAX_LEN as HEADER_MAX_LEN,
        },
        ipv4_data::Ipv4Data,
        ipv4_index::{BuildError as Ipv4IndexBuildError, Ipv4Index},
        ipv6_data::Ipv6Data,
        ipv6_index::{BuildError as Ipv6IndexBuildError, Ipv6Index},
        record::ParseError as RecordParseError,
    },
    record::Record,
};

//
#[derive(Debug)]
pub struct Database {
    pub header: Header,
    storage: Storage,
}

#[derive(Debug)]
pub enum Storage {
    Single(Ipv4Data, Ipv6Data),
}

impl Database {
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self, FromFileError> {
        let mut file = AsyncFsFile::open(path.as_ref())
            .await
            .map_err(FromFileError::FileOpenFailed)?;
        let mut buf = vec![0; 1024 * 8];

        //
        let header = {
            let mut parser = HeaderParser::new();
            let mut n_read = 0;
            let mut n_parsed = 0;
            loop {
                let n = file
                    .read(&mut buf[n_read..n_read + HEADER_MAX_LEN])
                    .await
                    .map_err(FromFileError::FileReadFailed)?;

                if n == 0 {
                    return Err(FromFileError::Other("header parsing is not completed"));
                }

                n_read += n;

                match parser
                    .parse(&mut Cursor::new(&buf[n_parsed..n_read]))
                    .map_err(FromFileError::HeaderParseFailed)?
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

        if header.total_size as u64
            != file
                .metadata()
                .await
                .map_err(FromFileError::FileReadMetadataFailed)?
                .len()
        {
            return Err(FromFileError::Other("file size mismatch"));
        }

        //
        let ipv4_index = {
            let mut ipv4_index_builder = Ipv4Index::builder();
            let mut n_max_appended = Ipv4Index::len() as usize;
            file.seek(SeekFrom::Start(
                header.ipv4_index_info.index_start as u64 - 1,
            ))
            .await
            .map_err(FromFileError::FileReadFailed)?;
            loop {
                let n = file
                    .read(&mut buf[..])
                    .await
                    .map_err(FromFileError::FileReadFailed)?;

                if n == 0 {
                    return Err(FromFileError::Other("ipv4_index building is not completed"));
                }

                if n < n_max_appended {
                    ipv4_index_builder.append(&buf[..n]);

                    n_max_appended -= n;
                    continue;
                } else {
                    ipv4_index_builder.append(&buf[..n_max_appended]);

                    break ipv4_index_builder
                        .finish()
                        .map_err(FromFileError::Ipv4IndexBuildFailed)?;
                }
            }
        };

        //
        let ipv6_index = {
            let mut ipv6_index_builder = Ipv6Index::builder();
            let mut n_max_appended = Ipv6Index::len() as usize;
            file.seek(SeekFrom::Start(
                header.ipv6_index_info.index_start as u64 - 1,
            ))
            .await
            .map_err(FromFileError::FileReadFailed)?;
            loop {
                let n = file
                    .read(&mut buf[..])
                    .await
                    .map_err(FromFileError::FileReadFailed)?;

                if n == 0 {
                    return Err(FromFileError::Other("ipv6_index building is not completed"));
                }

                if n < n_max_appended {
                    ipv6_index_builder.append(&buf[..n]);

                    n_max_appended -= n;
                    continue;
                } else {
                    ipv6_index_builder.append(&buf[..n_max_appended]);

                    break ipv6_index_builder
                        .finish()
                        .map_err(FromFileError::Ipv6IndexBuildFailed)?;
                }
            }
        };

        //
        let ipv4_data = Ipv4Data::new(
            TokioFile::open(path.as_ref())
                .await
                .map_err(FromFileError::FileOpenFailed)?,
            ipv4_index,
            header,
        );

        let ipv6_data = Ipv6Data::new(
            TokioFile::open(path)
                .await
                .map_err(FromFileError::FileOpenFailed)?,
            ipv6_index,
            header,
        );

        let storage = Storage::Single(ipv4_data, ipv6_data);

        Ok(Self { header, storage })
    }

    pub async fn ipv4_lookup(&mut self, addr: Ipv4Addr) -> Result<Option<Record>, LookupError> {
        match &mut self.storage {
            Storage::Single(ipv4_data, _) => ipv4_data.lookup(addr).await,
        }
    }

    pub async fn ipv6_lookup(&mut self, addr: Ipv6Addr) -> Result<Option<Record>, LookupError> {
        match &mut self.storage {
            Storage::Single(_, ipv6_data) => ipv6_data.lookup(addr).await,
        }
    }
}

//
#[derive(Debug)]
pub enum FromFileError {
    FileOpenFailed(IoError),
    FileReadFailed(IoError),
    FileReadMetadataFailed(IoError),
    HeaderParseFailed(HeaderParseError),
    Ipv4IndexBuildFailed(Ipv4IndexBuildError),
    Ipv6IndexBuildFailed(Ipv6IndexBuildError),
    Other(&'static str),
}

impl fmt::Display for FromFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FromFileError {}

//
#[derive(Debug)]
pub enum LookupError {
    FileSeekFailed(IoError),
    FileReadFailed(IoError),
    RecordParseFailed(RecordParseError),
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

    use std::{error, io::ErrorKind as IoErrorKind};

    use crate::bin_format::{
        TEST_20220401_BIN_FILES, TEST_20220401_BIN_IPV4_ADDRS, TEST_20220401_BIN_IPV6_ADDRS,
    };

    #[tokio::test]
    async fn test_lookup_20220401() -> Result<(), Box<dyn error::Error>> {
        for (path, _) in TEST_20220401_BIN_FILES {
            match Database::from_file(path).await {
                Ok(mut db) => {
                    for addr in TEST_20220401_BIN_IPV4_ADDRS {
                        let record = db.ipv4_lookup(Ipv4Addr::from(*addr)).await?;
                        assert!(record.is_some());
                    }

                    for addr in TEST_20220401_BIN_IPV6_ADDRS {
                        let record = db.ipv6_lookup(Ipv6Addr::from(*addr)).await?;
                        assert!(record.is_some());
                    }
                }
                Err(FromFileError::FileOpenFailed(err)) if err.kind() == IoErrorKind::NotFound => {
                    eprintln!("path:{}, err:{:?}", path, err);
                }
                Err(err) => panic!("path:{}, err:{:?}", path, err),
            }
        }

        Ok(())
    }
}
