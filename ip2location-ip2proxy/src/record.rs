//! [Ref](https://lite.ip2location.com/database/px11-ip-proxytype-country-region-city-isp-domain-usagetype-asn-lastseen-threat-residential-provider#database-fields)

use core::{convert::Infallible, fmt, str::FromStr};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use serde::{de, Deserialize, Deserializer};

use crate::{proxy_type::ProxyType, usage_type::UsageType};

//
pub const PX11_CSV_HEADER: &[&str] = &[
    "ip_from",
    "ip_to",
    "proxy_type",
    "country_code",
    "country_name",
    "region_name",
    "city_name",
    "isp",
    "domain",
    "usage_type",
    "asn",
    "as_name",
    "last_seen",
    "threat",
    "provider",
];

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    #[serde(deserialize_with = "ip_deserialize")]
    pub ip_from: IpAddr,
    #[serde(deserialize_with = "ip_deserialize")]
    pub ip_to: IpAddr,
    pub proxy_type: Option<ProxyType>,
    #[serde(with = "serde_field_with::to_and_from_string")]
    pub country_code: RecordValue,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub country_name: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub region_name: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub city_name: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub isp: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub domain: Option<RecordValue>,
    pub usage_type: Option<UsageType>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub asn: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub as_name: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub last_seen: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub threat: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub provider: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub residential: Option<RecordValue>,
}

fn ip_deserialize<'de, D>(deserializer: D) -> Result<IpAddr, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Box::<str>::deserialize(deserializer)?;
    if let Ok(v) = s.parse::<u32>() {
        Ok(Ipv4Addr::from(v).into())
    } else if let Ok(v) = s.parse::<Ipv6Addr>() {
        Ok(v.into())
    } else if let Ok(v) = s.parse::<Ipv4Addr>() {
        Ok(v.into())
    } else {
        Err(de::Error::custom(""))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecordValue {
    String(Box<str>),
    Usize(usize),
    Unknown,
}

impl Default for RecordValue {
    fn default() -> Self {
        Self::Unknown
    }
}

impl FromStr for RecordValue {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(Self::Unknown)
        } else if let Ok(v) = s.parse::<usize>() {
            Ok(Self::Usize(v))
        } else {
            Ok(Self::String(s.into()))
        }
    }
}

impl fmt::Display for RecordValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordValue::String(s) => write!(f, "{}", s),
            RecordValue::Usize(v) => write!(f, "{}", v),
            RecordValue::Unknown => write!(f, "-"),
        }
    }
}

impl Record {
    pub(crate) fn with_empty(ip_from: IpAddr, ip_to: IpAddr) -> Self {
        Self {
            ip_from,
            ip_to,
            proxy_type: Default::default(),
            country_code: Default::default(),
            country_name: Default::default(),
            region_name: Default::default(),
            city_name: Default::default(),
            isp: Default::default(),
            domain: Default::default(),
            usage_type: Default::default(),
            asn: Default::default(),
            as_name: Default::default(),
            last_seen: Default::default(),
            threat: Default::default(),
            residential: Default::default(),
            provider: Default::default(),
        }
    }
}
