use std::net::Ipv4Addr;

use super::{builder::Builder, inner::Inner};
use crate::{
    index::{ipv4_addr_position, INDEX_ELEMENT_LEN},
    records::PositionRange,
};

//
#[derive(Debug)]
pub struct V4Querier {
    inner: Inner,
}

impl From<Vec<u8>> for V4Querier {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            inner: Inner::new(bytes),
        }
    }
}

impl V4Querier {
    //
    pub fn builder() -> Builder {
        Builder::new()
    }

    //
    pub fn query(&self, ip: Ipv4Addr) -> PositionRange {
        let position = ipv4_addr_position(ip) as usize;

        let start = u32::from_ne_bytes(
            self.inner.bytes[position..position + INDEX_ELEMENT_LEN as usize]
                .try_into()
                .unwrap(),
        );
        let end = u32::from_ne_bytes(
            self.inner.bytes[position + INDEX_ELEMENT_LEN as usize
                ..position + INDEX_ELEMENT_LEN as usize + INDEX_ELEMENT_LEN as usize]
                .try_into()
                .unwrap(),
        );

        PositionRange::new(start, end)
    }
}
