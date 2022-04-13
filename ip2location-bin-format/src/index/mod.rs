use std::net::{Ipv4Addr, Ipv6Addr};

//
pub const INDEX_LEN: u32 = 524288;
pub const INDEX_ELEMENT_LEN: u32 = 4;

//
pub mod builder;
pub mod querier;

pub use querier::{V4Querier, V6Querier};

//
pub fn ipv4_addr_position(ip: Ipv4Addr) -> u32 {
    // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L190
    (u32::from(ip) >> 16) << 3
}

pub fn ipv6_addr_position(ip: Ipv6Addr) -> u32 {
    // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L217-L218
    ((ip.octets()[0] as u32) * 256 + (ip.octets()[1] as u32)) << 3
}

#[cfg(test)]
pub fn v4_index_len() -> u32 {
    // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L191-L192
    ipv4_addr_position(Ipv4Addr::new(255, 255, 255, 255)) + INDEX_ELEMENT_LEN + INDEX_ELEMENT_LEN
}

#[cfg(test)]
pub fn v6_index_len() -> u32 {
    // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L219-L220
    ipv6_addr_position(Ipv6Addr::new(
        0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
    )) + INDEX_ELEMENT_LEN
        + INDEX_ELEMENT_LEN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_len() {
        assert_eq!(v4_index_len(), INDEX_LEN);
        assert_eq!(v6_index_len(), INDEX_LEN);
    }
}
