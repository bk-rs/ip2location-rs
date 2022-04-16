//! https://lite.ip2location.com/ip2location-lite#db11-lite
//! https://lite.ip2location.com/ip2proxy-lite#px11-lite

use core::ops::{Deref, DerefMut};

use crate::header::schema::{SchemaSubType, SchemaType};

//
pub const RECORD_FIELD_LEN_WITHOUT_IP: u32 = 4;

//
// TODO,  12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
pub const RECORD_FIELDS_DBN_LIST: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

/*
https://github.com/ip2location/ip2location-go/blob/v9.2.0/ip2location.go#L123-L143

var country_position            = [26]uint8{0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,  2, 2,   2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2}
var region_position             = [26]uint8{0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,  3, 3,   3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  3}
var city_position               = [26]uint8{0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,  4, 4,   4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4}
var isp_position                = [26]uint8{0, 0, 3, 0, 5, 0, 7, 5, 7, 0, 8, 0, 9,  0, 9,   0,  9,  0,  9,  7,  9,  0,  9,  7,  9,  9}
var latitude_position           = [26]uint8{0, 0, 0, 0, 0, 5, 5, 0, 5, 5, 5, 5, 5,  5, 5,   5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5}
var longitude_position          = [26]uint8{0, 0, 0, 0, 0, 6, 6, 0, 6, 6, 6, 6, 6,  6, 6,   6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6}
var domain_position             = [26]uint8{0, 0, 0, 0, 0, 0, 0, 6, 8, 0, 9, 0, 10, 0, 10,  0, 10,  0, 10,  8, 10,  0, 10,  8, 10, 10}
var zipcode_position            = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 7,  0,  7,  7,  7,  0,  7,  0,  7,  7,  7,  0,  7,  7}
var timezone_position           = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8,  7,  8,  8,  8,  7,  8,  0,  8,  8,  8,  0,  8,  8}
var netspeed_position           = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  8, 11,  0, 11,  8, 11,  0, 11,  0, 11,  0, 11, 11}
var iddcode_position            = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  9, 12,  0, 12,  0, 12,  9, 12,  0, 12, 12}
var areacode_position           = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0, 10, 13,  0, 13,  0, 13, 10, 13,  0, 13, 13}
var weatherstationcode_position = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  9, 14,  0, 14,  0, 14,  0, 14, 14}
var weatherstationname_position = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0, 10, 15,  0, 15,  0, 15,  0, 15, 15}
var mcc_position                = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  9, 16,  0, 16,  9, 16, 16}
var mnc_position                = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0, 10, 17,  0, 17, 10, 17, 17}
var mobilebrand_position        = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0, 11, 18,  0, 18, 11, 18, 18}
var elevation_position          = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0, 11, 19,  0, 19, 19}
var usagetype_position          = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 12, 20, 20}
var addresstype_position        = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 21}
var category_position           = [26]uint8{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 22}

                                               1  2  3  4  5  6  7  8  9  10 11 12  13  14  15  16  17  18  19  20  21  22  23  24 25
*/

pub const RECORD_FIELDS_DB1: &[RecordField] = &[RecordField::IP, RecordField::COUNTRY];
pub const RECORD_FIELDS_DB2: &[RecordField] =
    &[RecordField::IP, RecordField::COUNTRY, RecordField::ISP];
pub const RECORD_FIELDS_DB3: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
];
pub const RECORD_FIELDS_DB4: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
];
pub const RECORD_FIELDS_DB5: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::LATITUDE,
    RecordField::LONGITUDE,
];
pub const RECORD_FIELDS_DB6: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::LATITUDE,
    RecordField::LONGITUDE,
    RecordField::ISP,
];
pub const RECORD_FIELDS_DB7: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
    RecordField::DOMAIN,
];
pub const RECORD_FIELDS_DB8: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::LATITUDE,
    RecordField::LONGITUDE,
    RecordField::ISP,
    RecordField::DOMAIN,
];
pub const RECORD_FIELDS_DB9: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::LATITUDE,
    RecordField::LONGITUDE,
    RecordField::ZIPCODE,
];
pub const RECORD_FIELDS_DB10: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::LATITUDE,
    RecordField::LONGITUDE,
    RecordField::ZIPCODE,
    RecordField::ISP,
    RecordField::DOMAIN,
];
pub const RECORD_FIELDS_DB11: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::LATITUDE,
    RecordField::LONGITUDE,
    RecordField::ZIPCODE,
    RecordField::TIMEZONE,
];

// TODO, RECORD_FIELDS_DB12 - RECORD_FIELDS_DB25

//
pub const RECORD_FIELDS_PXN_LIST: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

/*
https://github.com/ip2location/ip2proxy-go/blob/v3.3.2/ip2proxy.go#L99-L110

var countryPosition   = [12]uint8{0, 2, 3, 3, 3, 3, 3,  3,  3,  3,  3,  3}
var regionPosition    = [12]uint8{0, 0, 0, 4, 4, 4, 4,  4,  4,  4,  4,  4}
var cityPosition      = [12]uint8{0, 0, 0, 5, 5, 5, 5,  5,  5,  5,  5,  5}
var ispPosition       = [12]uint8{0, 0, 0, 0, 6, 6, 6,  6,  6,  6,  6,  6}
var proxyTypePosition = [12]uint8{0, 0, 2, 2, 2, 2, 2,  2,  2,  2,  2,  2}
var domainPosition    = [12]uint8{0, 0, 0, 0, 0, 7, 7,  7,  7,  7,  7,  7}
var usageTypePosition = [12]uint8{0, 0, 0, 0, 0, 0, 8,  8,  8,  8,  8,  8}
var asnPosition       = [12]uint8{0, 0, 0, 0, 0, 0, 0,  9,  9,  9,  9,  9}
var asPosition        = [12]uint8{0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10}
var lastSeenPosition  = [12]uint8{0, 0, 0, 0, 0, 0, 0,  0, 11, 11, 11, 11}
var threatPosition    = [12]uint8{0, 0, 0, 0, 0, 0, 0,  0,  0, 12, 12, 12}
var providerPosition  = [12]uint8{0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0, 13}

                                     1  2  3  4  5  6   7   8   9  10  11
*/

pub const RECORD_FIELDS_PX1: &[RecordField] = &[RecordField::IP, RecordField::COUNTRY];

pub const RECORD_FIELDS_PX2: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
];

pub const RECORD_FIELDS_PX3: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
];

pub const RECORD_FIELDS_PX4: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
];

pub const RECORD_FIELDS_PX5: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
    RecordField::DOMAIN,
];

pub const RECORD_FIELDS_PX6: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
    RecordField::DOMAIN,
    RecordField::USAGETYPE,
];

pub const RECORD_FIELDS_PX7: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
    RecordField::DOMAIN,
    RecordField::USAGETYPE,
    RecordField::ASN,
    RecordField::AS,
];

pub const RECORD_FIELDS_PX8: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
    RecordField::DOMAIN,
    RecordField::USAGETYPE,
    RecordField::ASN,
    RecordField::AS,
    RecordField::LASTSEEN,
];

pub const RECORD_FIELDS_PX9: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
    RecordField::DOMAIN,
    RecordField::USAGETYPE,
    RecordField::ASN,
    RecordField::AS,
    RecordField::LASTSEEN,
    RecordField::THREAT,
];

pub const RECORD_FIELDS_PX10: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
    RecordField::DOMAIN,
    RecordField::USAGETYPE,
    RecordField::ASN,
    RecordField::AS,
    RecordField::LASTSEEN,
    RecordField::THREAT,
];

pub const RECORD_FIELDS_PX11: &[RecordField] = &[
    RecordField::IP,
    RecordField::PROXYTYPE,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::ISP,
    RecordField::DOMAIN,
    RecordField::USAGETYPE,
    RecordField::ASN,
    RecordField::AS,
    RecordField::LASTSEEN,
    RecordField::THREAT,
    RecordField::PROVIDER,
];

//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordField {
    IP,
    // Common
    COUNTRY,
    REGION,
    CITY,
    ISP,
    DOMAIN,
    // IP2Location
    LATITUDE,
    LONGITUDE,
    ZIPCODE,
    TIMEZONE,
    // IP2Proxy
    PROXYTYPE,
    USAGETYPE,
    ASN,
    AS,
    LASTSEEN,
    THREAT,
    RESIDENTIAL,
    PROVIDER,
}

//
#[derive(Debug, Clone)]
pub struct RecordFields(Vec<RecordField>);

impl RecordFields {
    pub fn record_bytes_len_for_ipv4(&self) -> u32 {
        self.0.len() as u32 * RECORD_FIELD_LEN_WITHOUT_IP
    }

    pub fn record_bytes_len_for_ipv4_with_double_ip(&self) -> u32 {
        self.record_bytes_len_for_ipv4() + 4
    }

    pub fn records_bytes_len_for_ipv4(&self, n: u32) -> u32 {
        self.record_bytes_len_for_ipv4() * n
    }

    pub fn record_bytes_len_for_ipv6(&self) -> u32 {
        // 12 = 16 - 4
        self.0.len() as u32 * RECORD_FIELD_LEN_WITHOUT_IP + 12
    }

    pub fn record_bytes_len_for_ipv6_with_double_ip(&self) -> u32 {
        self.record_bytes_len_for_ipv6() + 16
    }

    pub fn records_bytes_len_for_ipv6(&self, n: u32) -> u32 {
        self.record_bytes_len_for_ipv6() * n
    }

    pub fn to_contents(&self) -> RecordFieldContents {
        assert_eq!(self.0[0], RecordField::IP);

        let inner = self.0[1..]
            .iter()
            .map(|x| match x {
                RecordField::IP => {
                    unreachable!()
                }
                RecordField::COUNTRY => {
                    RecordFieldContent::COUNTRY(0, Default::default(), Default::default())
                }
                RecordField::REGION => RecordFieldContent::REGION(0, Default::default()),
                RecordField::CITY => RecordFieldContent::CITY(0, Default::default()),
                RecordField::LATITUDE => RecordFieldContent::LATITUDE(0.0),
                RecordField::LONGITUDE => RecordFieldContent::LONGITUDE(0.0),
                RecordField::ZIPCODE => RecordFieldContent::ZIPCODE(0, Default::default()),
                RecordField::TIMEZONE => RecordFieldContent::TIMEZONE(0, Default::default()),
                RecordField::PROXYTYPE => RecordFieldContent::PROXYTYPE(0, Default::default()),
                RecordField::ISP => RecordFieldContent::ISP(0, Default::default()),
                RecordField::DOMAIN => RecordFieldContent::DOMAIN(0, Default::default()),
                RecordField::USAGETYPE => RecordFieldContent::USAGETYPE(0, Default::default()),
                RecordField::ASN => RecordFieldContent::ASN(0, Default::default()),
                RecordField::AS => RecordFieldContent::AS(0, Default::default()),
                RecordField::LASTSEEN => RecordFieldContent::LASTSEEN(0, Default::default()),
                RecordField::THREAT => RecordFieldContent::THREAT(0, Default::default()),
                RecordField::RESIDENTIAL => RecordFieldContent::RESIDENTIAL(0, Default::default()),
                RecordField::PROVIDER => RecordFieldContent::PROVIDER(0, Default::default()),
            })
            .collect::<Vec<_>>();

        RecordFieldContents(inner)
    }
}

impl Deref for RecordFields {
    type Target = Vec<RecordField>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<(SchemaType, SchemaSubType)> for RecordFields {
    type Error = SchemaSubType;

    fn try_from((r#type, sub_type): (SchemaType, SchemaSubType)) -> Result<Self, Self::Error> {
        match r#type {
            SchemaType::IP2Location | SchemaType::None => match sub_type.0 {
                1 => Ok(Self(RECORD_FIELDS_DB1.to_owned())),
                2 => Ok(Self(RECORD_FIELDS_DB2.to_owned())),
                3 => Ok(Self(RECORD_FIELDS_DB3.to_owned())),
                4 => Ok(Self(RECORD_FIELDS_DB4.to_owned())),
                5 => Ok(Self(RECORD_FIELDS_DB5.to_owned())),
                6 => Ok(Self(RECORD_FIELDS_DB6.to_owned())),
                7 => Ok(Self(RECORD_FIELDS_DB7.to_owned())),
                8 => Ok(Self(RECORD_FIELDS_DB8.to_owned())),
                9 => Ok(Self(RECORD_FIELDS_DB9.to_owned())),
                10 => Ok(Self(RECORD_FIELDS_DB10.to_owned())),
                11 => Ok(Self(RECORD_FIELDS_DB11.to_owned())),
                _ => Err(sub_type),
            },
            SchemaType::IP2Proxy => match sub_type.0 {
                1 => Ok(Self(RECORD_FIELDS_PX1.to_owned())),
                2 => Ok(Self(RECORD_FIELDS_PX2.to_owned())),
                3 => Ok(Self(RECORD_FIELDS_PX3.to_owned())),
                4 => Ok(Self(RECORD_FIELDS_PX4.to_owned())),
                5 => Ok(Self(RECORD_FIELDS_PX5.to_owned())),
                6 => Ok(Self(RECORD_FIELDS_PX6.to_owned())),
                7 => Ok(Self(RECORD_FIELDS_PX7.to_owned())),
                8 => Ok(Self(RECORD_FIELDS_PX8.to_owned())),
                9 => Ok(Self(RECORD_FIELDS_PX9.to_owned())),
                10 => Ok(Self(RECORD_FIELDS_PX10.to_owned())),
                11 => Ok(Self(RECORD_FIELDS_PX11.to_owned())),
                _ => Err(sub_type),
            },
        }
    }
}

//
#[derive(Debug, Clone)]
pub enum RecordFieldContent {
    // Common
    COUNTRY(u32, Box<str>, Box<str>),
    REGION(u32, Box<str>),
    CITY(u32, Box<str>),
    // IP2Location
    LATITUDE(f32),
    LONGITUDE(f32),
    ZIPCODE(u32, Box<str>),
    TIMEZONE(u32, Box<str>),
    // IP2Proxy
    PROXYTYPE(u32, Box<str>),
    ISP(u32, Box<str>),
    DOMAIN(u32, Box<str>),
    USAGETYPE(u32, Box<str>),
    ASN(u32, Box<str>),
    AS(u32, Box<str>),
    LASTSEEN(u32, Box<str>),
    THREAT(u32, Box<str>),
    RESIDENTIAL(u32, Box<str>),
    PROVIDER(u32, Box<str>),
}

#[derive(Debug, Clone)]
pub struct RecordFieldContents(Vec<RecordFieldContent>);

impl Deref for RecordFieldContents {
    type Target = Vec<RecordFieldContent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RecordFieldContents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RecordFieldContents {
    pub fn select(&mut self, record_fields: &[RecordField]) {
        self.0.retain(|x| match x {
            RecordFieldContent::COUNTRY(_, _, _) => record_fields.contains(&RecordField::COUNTRY),
            RecordFieldContent::REGION(_, _) => record_fields.contains(&RecordField::REGION),
            RecordFieldContent::CITY(_, _) => record_fields.contains(&RecordField::CITY),
            RecordFieldContent::LATITUDE(_) => record_fields.contains(&RecordField::LATITUDE),
            RecordFieldContent::LONGITUDE(_) => record_fields.contains(&RecordField::LONGITUDE),
            RecordFieldContent::ZIPCODE(_, _) => record_fields.contains(&RecordField::ZIPCODE),
            RecordFieldContent::TIMEZONE(_, _) => record_fields.contains(&RecordField::TIMEZONE),
            RecordFieldContent::PROXYTYPE(_, _) => record_fields.contains(&RecordField::PROXYTYPE),
            RecordFieldContent::ISP(_, _) => record_fields.contains(&RecordField::ISP),
            RecordFieldContent::DOMAIN(_, _) => record_fields.contains(&RecordField::DOMAIN),
            RecordFieldContent::USAGETYPE(_, _) => record_fields.contains(&RecordField::USAGETYPE),
            RecordFieldContent::ASN(_, _) => record_fields.contains(&RecordField::ASN),
            RecordFieldContent::AS(_, _) => record_fields.contains(&RecordField::AS),
            RecordFieldContent::LASTSEEN(_, _) => record_fields.contains(&RecordField::LASTSEEN),
            RecordFieldContent::THREAT(_, _) => record_fields.contains(&RecordField::THREAT),
            RecordFieldContent::RESIDENTIAL(_, _) => {
                record_fields.contains(&RecordField::RESIDENTIAL)
            }
            RecordFieldContent::PROVIDER(_, _) => record_fields.contains(&RecordField::PROVIDER),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_fields_prefix() {
        for n in RECORD_FIELDS_DBN_LIST {
            assert_eq!(
                RecordFields::try_from((SchemaType::IP2Location, SchemaSubType(*n)))
                    .unwrap()
                    .0[0],
                RecordField::IP
            )
        }

        for n in RECORD_FIELDS_PXN_LIST {
            assert_eq!(
                RecordFields::try_from((SchemaType::IP2Proxy, SchemaSubType(*n)))
                    .unwrap()
                    .0[0],
                RecordField::IP
            )
        }
    }
}
