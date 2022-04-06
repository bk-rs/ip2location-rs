//! [Ref](https://lite.ip2location.com/database/px6-ip-proxytype-country-region-city-isp-domain-usagetype#database-fields)

use serde_enum_str::Deserialize_enum_str;

//
#[derive(Deserialize_enum_str, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum UsageType {
    COM,
    ORG,
    GOV,
    MIL,
    EDU,
    LIB,
    CDN,
    ISP,
    MOB,
    DCH,
    SES,
    RSV,
    #[serde(other)]
    Other(Box<str>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!("ISP".parse::<UsageType>().unwrap(), UsageType::ISP);
    }
}
