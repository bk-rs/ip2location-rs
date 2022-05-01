//! [Ref](https://lite.ip2location.com/database/px11-ip-proxytype-country-region-city-isp-domain-usagetype-asn-lastseen-threat-residential-provider#database-fields)

use std::net::IpAddr;

use country_code::CountryCode;
use ip2location_bin_format::content::UNKNOWN_STR;

use crate::{proxy_type::ProxyType, usage_type::UsageType};

//
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Record {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "ip_deserialize"))]
    pub ip_from: IpAddr,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "ip_deserialize"))]
    pub ip_to: IpAddr,
    pub proxy_type: Option<ProxyType>,
    pub country_code: CountryCode,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub country_name: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub region_name: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub city_name: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub isp: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub domain: Option<Box<str>>,
    pub usage_type: Option<UsageType>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_usize_deserialize")
    )]
    pub asn: Option<usize>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub as_name: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub last_seen: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub threat: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub provider: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub residential: Option<Box<str>>,
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
fn option_box_str_deserialize<'de, D>(deserializer: D) -> Result<Option<Box<str>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;

    let s = Box::<str>::deserialize(deserializer)?;
    if s == UNKNOWN_STR.into() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

#[cfg(feature = "serde")]
fn option_usize_deserialize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;

    let s = Box::<str>::deserialize(deserializer)?;
    if s == UNKNOWN_STR.into() {
        Ok(None)
    } else {
        match s.parse::<usize>() {
            Ok(v) => Ok(Some(v)),
            Err(err) => Err(serde::de::Error::custom(err.to_string())),
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

//
pub(crate) struct OptionRecord(pub(crate) Option<Record>);

impl
    TryFrom<(
        IpAddr,
        IpAddr,
        ip2location_bin_format::record_field::RecordFieldContents,
    )> for OptionRecord
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
                    if let Some(v) = v {
                        record.country_code = v
                            .parse::<CountryCode>()
                            .map_err(|err| Box::<str>::from(err.to_string()))?;
                    } else {
                        return Ok(OptionRecord(None));
                    }

                    record.country_name = v_name.to_owned();
                }
                RecordFieldContent::REGION(_, v) => {
                    record.region_name = v.to_owned();
                }
                RecordFieldContent::CITY(_, v) => {
                    record.city_name = v.to_owned();
                }
                RecordFieldContent::ISP(_, v) => {
                    record.isp = v.to_owned();
                }
                RecordFieldContent::DOMAIN(_, v) => {
                    record.domain = v.to_owned();
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
                    if let Some(v) = v {
                        let v = v
                            .parse::<ProxyType>()
                            .map_err(|err| Box::<str>::from(err.to_string()))?;
                        record.proxy_type = Some(v);
                    }
                }
                RecordFieldContent::USAGETYPE(_, v) => {
                    if let Some(v) = v {
                        let v = v
                            .parse::<UsageType>()
                            .map_err(|err| Box::<str>::from(err.to_string()))?;
                        record.usage_type = Some(v);
                    }
                }
                RecordFieldContent::ASN(_, v) => {
                    if let Some(v) = v {
                        let v = v
                            .parse::<usize>()
                            .map_err(|err| Box::<str>::from(err.to_string()))?;
                        record.asn = Some(v);
                    }
                }
                RecordFieldContent::AS(_, v) => {
                    record.as_name = v.to_owned();
                }
                RecordFieldContent::LASTSEEN(_, v) => {
                    record.last_seen = v.to_owned();
                }
                RecordFieldContent::THREAT(_, v) => {
                    record.threat = v.to_owned();
                }
                RecordFieldContent::RESIDENTIAL(_, v) => {
                    record.residential = v.to_owned();
                }
                RecordFieldContent::PROVIDER(_, v) => {
                    record.provider = v.to_owned();
                }
            }
        }

        Ok(OptionRecord(Some(record)))
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
    ProxyType,
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
            RecordField::CountryCodeAndName => Self::COUNTRY,
            RecordField::RegionName => Self::REGION,
            RecordField::CityName => Self::CITY,
            RecordField::Isp => Self::ISP,
            RecordField::Domain => Self::DOMAIN,
            //
            RecordField::ProxyType => Self::PROXYTYPE,
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
