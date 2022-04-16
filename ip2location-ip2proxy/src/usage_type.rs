//! [Ref](https://lite.ip2location.com/database/px6-ip-proxytype-country-region-city-isp-domain-usagetype#database-fields)

//
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde_enum_str::Deserialize_enum_str))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
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
    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    Unknown,
    #[cfg_attr(feature = "serde", serde(other))]
    Other(Box<str>),
}

#[cfg(not(feature = "serde"))]
impl core::str::FromStr for UsageType {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COM" => Ok(Self::COM),
            "ORG" => Ok(Self::ORG),
            "GOV" => Ok(Self::GOV),
            "MIL" => Ok(Self::MIL),
            "EDU" => Ok(Self::EDU),
            "LIB" => Ok(Self::LIB),
            "CDN" => Ok(Self::CDN),
            "ISP" => Ok(Self::ISP),
            "MOB" => Ok(Self::MOB),
            "DCH" => Ok(Self::DCH),
            "SES" => Ok(Self::SES),
            "RSV" => Ok(Self::RSV),
            "-" => Ok(Self::Unknown),
            s => Ok(Self::Other(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!("ISP".parse::<UsageType>().unwrap(), UsageType::ISP);
    }
}
