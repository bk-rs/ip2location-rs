use core::fmt;
use std::net::Ipv6Addr;

//
pub struct Ipv6Index {
    bytes: Vec<u8>,
}

impl fmt::Debug for Ipv6Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ipv6Index")
            .field("bytes", &self.bytes.len())
            .finish()
    }
}

impl Ipv6Index {
    pub fn ipv6_addr_index(ipv6_addr: Ipv6Addr) -> u32 {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L217-L218
        ((ipv6_addr.octets()[0] as u32) * 256 + (ipv6_addr.octets()[1] as u32)) << 3
    }

    pub fn len() -> u32 {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L219-L220
        Self::ipv6_addr_index(Ipv6Addr::new(
            0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
        )) + 4
            + 4
    }

    //
    pub fn builder() -> Builder {
        Builder::new()
    }

    //
    pub fn low_and_high(&self, ipv6_addr: Ipv6Addr) -> (u32, u32) {
        let index = Self::ipv6_addr_index(ipv6_addr) as usize;

        let low = u32::from_ne_bytes(self.bytes[index..index + 4].try_into().unwrap());
        let high = u32::from_ne_bytes(self.bytes[index + 4..index + 8].try_into().unwrap());

        (low, high)
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
            bytes: Vec::with_capacity(Ipv6Index::len() as usize),
        }
    }

    pub fn append(&mut self, slice: &[u8]) {
        self.bytes.extend_from_slice(slice);
    }

    pub fn finish(self) -> Result<Ipv6Index, BuildError> {
        if self.bytes.len() != Ipv6Index::len() as usize {
            return Err(BuildError::LenMismatch);
        }

        Ok(Ipv6Index { bytes: self.bytes })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        assert_eq!(
            Ipv6Index::ipv6_addr_index(Ipv6Addr::new(
                0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
            )),
            524280
        );

        assert_eq!(Ipv6Index::len(), 524288);
    }
}
