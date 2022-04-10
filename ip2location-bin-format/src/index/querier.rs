use core::fmt;
use std::net::IpAddr;

use super::{ipv4_addr_position, ipv6_addr_position, INDEX_ELEMENT_LEN, INDEX_LEN};
use crate::records::PositionRange;

//
#[derive(Debug)]
pub struct Querier {
    bytes: Vec<u8>,
}

impl Querier {
    //
    pub fn builder() -> Builder {
        Builder::new()
    }

    //
    pub fn record_position_range(&self, ip: IpAddr) -> PositionRange {
        let position = match ip {
            IpAddr::V4(ip) => ipv4_addr_position(ip),
            IpAddr::V6(ip) => ipv6_addr_position(ip),
        } as usize;

        let start = u32::from_ne_bytes(
            self.bytes[position..position + INDEX_ELEMENT_LEN as usize]
                .try_into()
                .unwrap(),
        );
        let end = u32::from_ne_bytes(
            self.bytes[position + INDEX_ELEMENT_LEN as usize
                ..position + INDEX_ELEMENT_LEN as usize + INDEX_ELEMENT_LEN as usize]
                .try_into()
                .unwrap(),
        );

        PositionRange::new(start, end)
    }
}

//
//
//
pub struct Builder {
    bytes: Vec<u8>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            bytes: Vec::with_capacity(INDEX_LEN as usize),
        }
    }

    pub fn append(&mut self, slice: &[u8]) {
        self.bytes.extend_from_slice(slice);
    }

    pub fn finish(self) -> Result<Querier, BuildError> {
        if self.bytes.len() != INDEX_LEN as usize {
            return Err(BuildError::LenMismatch);
        }

        Ok(Querier { bytes: self.bytes })
    }
}

//
#[derive(Debug)]
pub enum BuildError {
    LenMismatch,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BuildError {}
