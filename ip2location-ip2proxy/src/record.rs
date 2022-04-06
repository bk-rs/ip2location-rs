//! [Ref](https://lite.ip2location.com/database/px11-ip-proxytype-country-region-city-isp-domain-usagetype-asn-lastseen-threat-residential-provider#database-fields)

use std::net::IpAddr;

use serde::Deserialize;

use crate::{proxy_type::ProxyType, usage_type::UsageType};

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub ip_from: IpAddr,
    pub ip_to: IpAddr,
    pub proxy_type: Option<ProxyType>,
    pub country_code: Box<str>,
    pub country_name: Option<Box<str>>,
    pub region_name: Option<Box<str>>,
    pub city_name: Option<Box<str>>,
    pub isp: Option<Box<str>>,
    pub domain: Option<Box<str>>,
    pub usage_type: Option<UsageType>,
    pub asn: Option<usize>,
    pub r#as: Option<Box<str>>,
    pub last_seen: Option<usize>,
    pub threat: Option<Box<str>>,
    pub provider: Option<Box<str>>,
}
