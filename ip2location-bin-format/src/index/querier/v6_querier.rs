use std::net::Ipv6Addr;

use super::{builder::Builder, inner::Inner};
use crate::{
    index::{ipv6_addr_position, INDEX_ELEMENT_LEN},
    records::PositionRange,
};

//
#[derive(Debug)]
pub struct V6Querier {
    inner: Inner,
}

impl From<Vec<u8>> for V6Querier {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            inner: Inner::new(bytes),
        }
    }
}

impl V6Querier {
    //
    pub fn builder() -> Builder {
        Builder::new()
    }

    //
    pub fn query(&self, ip: Ipv6Addr) -> PositionRange {
        debug_assert!(ip.to_ipv4().is_none());

        let position = ipv6_addr_position(ip) as usize;

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
