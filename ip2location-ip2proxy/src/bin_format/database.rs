use core::{fmt, ops::ControlFlow};
use std::{
    io::{Cursor, Error as IoError, SeekFrom},
    path::Path,
};

use async_fs::File as AsyncFsFile;
use futures_util::{AsyncReadExt as _, AsyncSeekExt as _};
use tokio::fs::File as TokioFile;

use crate::bin_format::{
    header::{Header, ParseError as HeaderParseError, Parser as HeaderParser},
    ipv4_data::Ipv4Data,
    ipv4_index::{BuildError as Ipv4IndexBuildError, Ipv4Index},
    ipv6_data::Ipv6Data,
};

//
#[derive(Debug)]
pub struct Database {
    header: Header,
    ipv4_index: Ipv4Index,
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
        let mut parser = HeaderParser::new();
        let mut n_read = 0;
        let mut n_parsed = 0;
        let header = loop {
            let n = file
                .read(&mut buf[n_parsed..1024])
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
        };

        //
        let mut ipv4_index_builder = Ipv4Index::builder();
        let mut n_max_appended = Ipv4Index::len() as usize;
        file.seek(SeekFrom::Start(header.ipv4_index_info.index_start as u64))
            .await
            .map_err(FromFileError::FileReadFailed)?;
        let ipv4_index = loop {
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
        };

        //
        let ipv4_data = Ipv4Data::new(
            TokioFile::open(path.as_ref())
                .await
                .map_err(FromFileError::FileOpenFailed)?,
        );

        let ipv6_data = Ipv6Data::new(
            TokioFile::open(path)
                .await
                .map_err(FromFileError::FileOpenFailed)?,
        );

        let storage = Storage::Single(ipv4_data, ipv6_data);

        Ok(Self {
            header,
            ipv4_index,
            storage,
        })
    }
}

//
#[derive(Debug)]
pub enum FromFileError {
    FileOpenFailed(IoError),
    FileReadFailed(IoError),
    HeaderParseFailed(HeaderParseError),
    Ipv4IndexBuildFailed(Ipv4IndexBuildError),
    Other(&'static str),
}

impl fmt::Display for FromFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FromFileError {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[tokio::test]
    async fn test_from_file_20220401() -> Result<(), Box<dyn error::Error>> {
        let path = "data/20220401/IP2PROXY-LITE-PX1.BIN";

        let db = Database::from_file(path).await?;

        println!("db: {:?}", db);

        Ok(())
    }
}
