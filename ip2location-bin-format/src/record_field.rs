//! https://lite.ip2location.com/ip2location-lite#db11-lite
//! https://lite.ip2location.com/ip2proxy-lite#px11-lite

use core::ops::Deref;

use crate::header::schema::{SchemaSubType, SchemaType};

//
pub const RECORD_FIELD_LEN_WITHOUT_IP: u32 = 4;

//
pub const RECORD_FIELDS_DBN_LIST: &[u8] = &[1, 3, 5, 9, 11];

pub const RECORD_FIELDS_DB1: &[RecordField] = &[RecordField::IP, RecordField::COUNTRY];
pub const RECORD_FIELDS_DB3: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
];
pub const RECORD_FIELDS_DB5: &[RecordField] = &[
    RecordField::IP,
    RecordField::COUNTRY,
    RecordField::REGION,
    RecordField::CITY,
    RecordField::LATITUDE,
    RecordField::LONGITUDE,
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

//
pub const RECORD_FIELDS_PXN_LIST: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

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
    // IP2Location
    LATITUDE,
    LONGITUDE,
    ZIPCODE,
    TIMEZONE,
    // IP2Proxy
    PROXYTYPE,
    ISP,
    DOMAIN,
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

    pub fn field_and_len_list_without_ip(&self) -> Vec<(&RecordField, u32)> {
        assert_eq!(self.0[0], RecordField::IP);
        self.0[1..]
            .iter()
            .map(|x| (x, RECORD_FIELD_LEN_WITHOUT_IP))
            .collect::<Vec<_>>()
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
                3 => Ok(Self(RECORD_FIELDS_DB3.to_owned())),
                5 => Ok(Self(RECORD_FIELDS_DB5.to_owned())),
                9 => Ok(Self(RECORD_FIELDS_DB9.to_owned())),
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
