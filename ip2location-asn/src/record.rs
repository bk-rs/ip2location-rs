//! [Ref](https://lite.ip2location.com/database-asn#database-fields)

use std::net::IpAddr;

use ipnetwork::IpNetwork;

//
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Record {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "ip_deserialize"))]
    pub ip_from: IpAddr,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "ip_deserialize"))]
    pub ip_to: IpAddr,
    pub cidr: IpNetwork,
    pub asn: Option<u32>,
    pub r#as: Option<String>,
}

#[cfg(feature = "serde")]
fn ip_deserialize<'de, D>(deserializer: D) -> Result<IpAddr, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::net::{Ipv4Addr, Ipv6Addr};

    use serde::Deserialize as _;

    let s = Box::<str>::deserialize(deserializer)?;
    if let Ok(v) = s.parse::<u32>() {
        Ok(Ipv4Addr::from(v).into())
    } else if let Ok(v) = s.parse::<u128>() {
        Ok(Ipv6Addr::from(v).into())
    } else if let Ok(v) = s.parse::<Ipv4Addr>() {
        Ok(v.into())
    } else if let Ok(v) = s.parse::<Ipv6Addr>() {
        Ok(v.into())
    } else {
        Err(serde::de::Error::custom(""))
    }
}
