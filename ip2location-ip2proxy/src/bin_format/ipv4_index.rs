use core::fmt;
use std::net::Ipv4Addr;

//
pub struct Ipv4Index {
    bytes: Vec<u8>,
}

impl fmt::Debug for Ipv4Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ipv4Index")
            .field("bytes", &self.bytes.len())
            .finish()
    }
}

impl Ipv4Index {
    pub fn ipv4_addr_index(ipv4_addr: Ipv4Addr) -> u32 {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L190
        (u32::from(ipv4_addr) >> 16) << 3
    }

    pub fn len() -> u32 {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L191-L192
        Self::ipv4_addr_index(Ipv4Addr::new(255, 255, 255, 255)) + 4 + 4
    }

    //
    pub fn builder() -> Builder {
        Builder::new()
    }

    //
    pub fn low_and_high(&self, ipv4_addr: Ipv4Addr) -> (u32, u32) {
        let index = Self::ipv4_addr_index(ipv4_addr) as usize;

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
            bytes: Vec::with_capacity(Ipv4Index::len() as usize),
        }
    }

    pub fn append(&mut self, slice: &[u8]) {
        self.bytes.extend_from_slice(slice);
    }

    pub fn finish(self) -> Result<Ipv4Index, BuildError> {
        if self.bytes.len() != Ipv4Index::len() as usize {
            return Err(BuildError::LenMismatch);
        }

        Ok(Ipv4Index { bytes: self.bytes })
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
            Ipv4Index::ipv4_addr_index(Ipv4Addr::new(255, 255, 255, 255)),
            524280
        );

        assert_eq!(Ipv4Index::len(), 524288);
    }
}
