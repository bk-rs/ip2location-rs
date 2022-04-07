//! [Ref](https://lite.ip2location.com/database-download#ip2proxy-database)

use crate::bin_format::header::Type;

//
#[derive(Debug, Clone, Copy)]
pub enum Field {
    IP,
    COUNTRY,
    PROXYTYPE,
    REGION,
    CITY,
    ISP,
    DOMAIN,
    USAGETYPE,
    ASN,
    LASTSEEN,
    THREAT,
    RESIDENTIAL,
    PROVIDER,
}

impl Field {
    pub fn fields_by_type(r#type: &Type) -> Vec<Self> {
        match r#type {
            Type::PX1 => vec![Field::IP, Field::COUNTRY],
            Type::PX2 => vec![Field::IP, Field::PROXYTYPE, Field::COUNTRY],
            Type::PX3 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
            ],
            Type::PX4 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
                Field::ISP,
            ],
            Type::PX5 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
                Field::ISP,
                Field::DOMAIN,
            ],
            Type::PX6 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
                Field::ISP,
                Field::DOMAIN,
                Field::USAGETYPE,
            ],
            Type::PX7 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
                Field::ISP,
                Field::DOMAIN,
                Field::USAGETYPE,
                Field::ASN,
            ],
            Type::PX8 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
                Field::ISP,
                Field::DOMAIN,
                Field::USAGETYPE,
                Field::ASN,
                Field::LASTSEEN,
            ],
            Type::PX9 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
                Field::ISP,
                Field::DOMAIN,
                Field::USAGETYPE,
                Field::ASN,
                Field::LASTSEEN,
                Field::THREAT,
            ],
            Type::PX10 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
                Field::ISP,
                Field::DOMAIN,
                Field::USAGETYPE,
                Field::ASN,
                Field::LASTSEEN,
                Field::THREAT,
                Field::RESIDENTIAL,
            ],
            Type::PX11 => vec![
                Field::IP,
                Field::PROXYTYPE,
                Field::COUNTRY,
                Field::REGION,
                Field::CITY,
                Field::ISP,
                Field::DOMAIN,
                Field::USAGETYPE,
                Field::ASN,
                Field::LASTSEEN,
                Field::THREAT,
                Field::RESIDENTIAL,
                Field::PROVIDER,
            ],
        }
    }
}
