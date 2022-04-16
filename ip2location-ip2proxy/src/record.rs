//! [Ref](https://lite.ip2location.com/database/px11-ip-proxytype-country-region-city-isp-domain-usagetype-asn-lastseen-threat-residential-provider#database-fields)

use core::{convert::Infallible, fmt, str::FromStr};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use serde::{de, Deserialize, Deserializer};

use crate::{proxy_type::ProxyType, usage_type::UsageType};

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
    } else if let Ok(v) = s.parse::<u128>() {
        Ok(Ipv6Addr::from(v).into())
    } else if let Ok(v) = s.parse::<Ipv4Addr>() {
        Ok(v.into())
    } else if let Ok(v) = s.parse::<Ipv6Addr>() {
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

impl RecordValue {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
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

impl
    TryFrom<(
        IpAddr,
        IpAddr,
        ip2location_bin_format::record_field::RecordFieldContents,
    )> for Record
{
    type Error = Box<str>;

    fn try_from(
        (ip_from, ip_to, record_field_contents): (
            IpAddr,
            IpAddr,
            ip2location_bin_format::record_field::RecordFieldContents,
        ),
    ) -> Result<Self, Self::Error> {
        use ip2location_bin_format::record_field::RecordFieldContent;

        let mut record = Record::with_empty(ip_from, ip_to);

        for record_field_content in record_field_contents.iter() {
            match record_field_content {
                RecordFieldContent::COUNTRY(_, v, v_name) => {
                    record.country_code = v.parse().expect("unreachable");
                    record.country_name = Some(v_name.parse().expect("unreachable"));
                }
                RecordFieldContent::REGION(_, v) => {
                    record.region_name = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::CITY(_, v) => {
                    record.city_name = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::ISP(_, v) => {
                    record.isp = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::DOMAIN(_, v) => {
                    record.domain = Some(v.parse().expect("unreachable"));
                }
                //
                RecordFieldContent::LATITUDE(_) => {
                    return Err("Unknown field LATITUDE".into());
                }
                RecordFieldContent::LONGITUDE(_) => {
                    return Err("Unknown field LONGITUDE".into());
                }
                RecordFieldContent::ZIPCODE(_, _) => {
                    return Err("Unknown field ZIPCODE".into());
                }
                RecordFieldContent::TIMEZONE(_, _) => {
                    return Err("Unknown field TIMEZONE".into());
                }
                RecordFieldContent::NETSPEED(_, _) => {
                    return Err("Unknown field NETSPEED".into());
                }
                //
                RecordFieldContent::PROXYTYPE(_, v) => {
                    let v = v
                        .parse::<ProxyType>()
                        .map_err(|err| Box::<str>::from(err.to_string()))?;
                    record.proxy_type = Some(v);
                }
                RecordFieldContent::USAGETYPE(_, v) => {
                    let v = v
                        .parse::<UsageType>()
                        .map_err(|err| Box::<str>::from(err.to_string()))?;
                    record.usage_type = Some(v);
                }
                RecordFieldContent::ASN(_, v) => {
                    record.asn = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::AS(_, v) => {
                    record.as_name = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::LASTSEEN(_, v) => {
                    record.last_seen = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::THREAT(_, v) => {
                    record.threat = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::RESIDENTIAL(_, v) => {
                    record.residential = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::PROVIDER(_, v) => {
                    record.provider = Some(v.parse().expect("unreachable"));
                }
            }
        }

        Ok(record)
    }
}

//
//
//
#[derive(Debug, Clone, Copy)]
pub enum RecordField {
    ProxyType,
    CountryCodeAndName,
    RegionName,
    CityName,
    Isp,
    Domain,
    UsageType,
    Asn,
    AsName,
    LastSeen,
    Threat,
    Provider,
    Residential,
}

impl From<&RecordField> for ip2location_bin_format::record_field::RecordField {
    fn from(x: &RecordField) -> Self {
        match x {
            RecordField::ProxyType => Self::PROXYTYPE,
            RecordField::CountryCodeAndName => Self::COUNTRY,
            RecordField::RegionName => Self::REGION,
            RecordField::CityName => Self::CITY,
            RecordField::Isp => Self::ISP,
            RecordField::Domain => Self::DOMAIN,
            RecordField::UsageType => Self::USAGETYPE,
            RecordField::Asn => Self::ASN,
            RecordField::AsName => Self::AS,
            RecordField::LastSeen => Self::LASTSEEN,
            RecordField::Threat => Self::THREAT,
            RecordField::Provider => Self::PROVIDER,
            RecordField::Residential => Self::RESIDENTIAL,
        }
    }
}
