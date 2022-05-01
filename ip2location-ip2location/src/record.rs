//! [Ref](https://lite.ip2location.com/database/db11-ip-country-region-city-latitude-longitude-zipcode-timezone#database-fields)

use std::net::IpAddr;

//
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Record {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "ip_deserialize"))]
    pub ip_from: IpAddr,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "ip_deserialize"))]
    pub ip_to: IpAddr,
    pub country_code: Box<str>,
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
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub zip_code: Option<Box<str>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub time_zone: Option<Box<str>>,
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
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "option_box_str_deserialize")
    )]
    pub net_speed: Option<Box<str>>,
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
    if s == "-".into() {
        Ok(None)
    } else {
        Ok(Some(s))
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
                    record.country_code = v.to_owned();
                    record.country_name = Some(v_name.to_owned());
                }
                RecordFieldContent::REGION(_, v) => {
                    record.region_name = Some(v.to_owned());
                }
                RecordFieldContent::CITY(_, v) => {
                    record.city_name = Some(v.to_owned());
                }
                RecordFieldContent::ISP(_, v) => {
                    record.isp = Some(v.to_owned());
                }
                RecordFieldContent::DOMAIN(_, v) => {
                    record.domain = Some(v.to_owned());
                }
                //
                RecordFieldContent::LATITUDE(v) => {
                    record.latitude = Some(*v);
                }
                RecordFieldContent::LONGITUDE(v) => {
                    record.longitude = Some(*v);
                }
                RecordFieldContent::ZIPCODE(_, v) => {
                    record.zip_code = Some(v.to_owned());
                }
                RecordFieldContent::TIMEZONE(_, v) => {
                    record.time_zone = Some(v.to_owned());
                }
                RecordFieldContent::NETSPEED(_, v) => {
                    record.net_speed = Some(v.to_owned());
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
