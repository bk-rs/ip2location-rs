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
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_asn_deserialize")
    )]
    pub asn: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub r#as: Option<Box<str>>,
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

#[cfg(feature = "serde")]
fn option_asn_deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;

    use crate::UNKNOWN_STR;

    let s = Box::<str>::deserialize(deserializer)?;
    if s == UNKNOWN_STR.into() {
        Ok(None)
    } else {
        Ok(Some(s.parse::<u32>().map_err(|err| {
            serde::de::Error::custom(err.to_string())
        })?))
    }
}

#[cfg(feature = "serde")]
fn option_box_str_deserialize<'de, D>(deserializer: D) -> Result<Option<Box<str>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;

    use crate::UNKNOWN_STR;

    let s = Box::<str>::deserialize(deserializer)?;
    if s == UNKNOWN_STR.into() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}
