use std::io::Error as IoError;

//
#[derive(Debug)]
pub enum Error {
    SeekFailed(IoError),
    ReadFailed(IoError),
    MaxDepthReached,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
