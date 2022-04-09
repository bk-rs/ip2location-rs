//! [Ref](https://lite.ip2location.com/database/px11-ip-proxytype-country-region-city-isp-domain-usagetype-asn-lastseen-threat-residential-provider#database-fields)

use core::{convert::Infallible, fmt, str::FromStr};
use std::net::IpAddr;

use serde::Deserialize;

use crate::{proxy_type::ProxyType, usage_type::UsageType};

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub ip_from: IpAddr,
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

#[derive(Debug, Clone)]
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
