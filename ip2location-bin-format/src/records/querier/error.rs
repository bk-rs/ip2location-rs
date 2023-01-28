use core::fmt;
use std::io::Error as IoError;

//
#[derive(Debug)]
pub enum Error {
    SeekFailed(IoError),
    ReadFailed(IoError),
    MaxDepthReached,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
