//! [Ref](https://lite.ip2location.com/database/db11-ip-country-region-city-latitude-longitude-zipcode-timezone#database-fields)

use core::{convert::Infallible, fmt, str::FromStr};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use serde::{de, Deserialize, Deserializer};

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    #[serde(deserialize_with = "ip_deserialize")]
    pub ip_from: IpAddr,
    #[serde(deserialize_with = "ip_deserialize")]
    pub ip_to: IpAddr,
    #[serde(with = "serde_field_with::to_and_from_string")]
    pub country_code: RecordValue,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub country_name: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub region_name: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub city_name: Option<RecordValue>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub zip_code: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub time_zone: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub isp: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub domain: Option<RecordValue>,
    #[serde(default, with = "serde_field_with::to_and_from_string_option")]
    pub net_speed: Option<RecordValue>,
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
        } else {
            Ok(Self::String(s.into()))
        }
    }
}

impl fmt::Display for RecordValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordValue::String(s) => write!(f, "{}", s),
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
            country_code: Default::default(),
            country_name: Default::default(),
            region_name: Default::default(),
            city_name: Default::default(),
            latitude: Default::default(),
            longitude: Default::default(),
            zip_code: Default::default(),
            time_zone: Default::default(),
            isp: Default::default(),
            domain: Default::default(),
            net_speed: Default::default(),
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
                RecordFieldContent::LATITUDE(v) => {
                    record.latitude = Some(*v);
                }
                RecordFieldContent::LONGITUDE(v) => {
                    record.longitude = Some(*v);
                }
                RecordFieldContent::ZIPCODE(_, v) => {
                    record.zip_code = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::TIMEZONE(_, v) => {
                    record.time_zone = Some(v.parse().expect("unreachable"));
                }
                RecordFieldContent::NETSPEED(_, v) => {
                    record.net_speed = Some(v.parse().expect("unreachable"));
                }
                //
                RecordFieldContent::PROXYTYPE(_, _) => {
                    return Err("Unknown field PROXYTYPE".into());
                }
                RecordFieldContent::USAGETYPE(_, _) => {
                    return Err("Unknown field USAGETYPE".into());
                }
                RecordFieldContent::ASN(_, _) => {
                    return Err("Unknown field ASN".into());
                }
                RecordFieldContent::AS(_, _) => {
                    return Err("Unknown field AS".into());
                }
                RecordFieldContent::LASTSEEN(_, _) => {
                    return Err("Unknown field LASTSEEN".into());
                }
                RecordFieldContent::THREAT(_, _) => {
                    return Err("Unknown field THREAT".into());
                }
                RecordFieldContent::RESIDENTIAL(_, _) => {
                    return Err("Unknown field RESIDENTIAL".into());
                }
                RecordFieldContent::PROVIDER(_, _) => {
                    return Err("Unknown field PROVIDER".into());
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
    CountryCodeAndName,
    RegionName,
    CityName,
    Isp,
    Domain,
    //
    Latitude,
    Longitude,
    ZipCode,
    TimeZone,
    NetSpeed,
}

impl From<&RecordField> for ip2location_bin_format::record_field::RecordField {
    fn from(x: &RecordField) -> Self {
        match x {
            RecordField::CountryCodeAndName => Self::COUNTRY,
            RecordField::RegionName => Self::REGION,
            RecordField::CityName => Self::CITY,
            RecordField::Isp => Self::ISP,
            RecordField::Domain => Self::DOMAIN,
            //
            RecordField::Latitude => Self::LATITUDE,
            RecordField::Longitude => Self::LONGITUDE,
            RecordField::ZipCode => Self::ZIPCODE,
            RecordField::TimeZone => Self::TIMEZONE,
            RecordField::NetSpeed => Self::NETSPEED,
        }
    }
}
