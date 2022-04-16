//! [Ref](https://lite.ip2location.com/database/px2-ip-proxytype-country#proxy-type)

//
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde_enum_str::Deserialize_enum_str))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
pub enum ProxyType {
    VPN,
    TOR,
    DCH,
    PUB,
    WEB,
    SES,
    RES,
    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    Unknown,
    #[cfg_attr(feature = "serde", serde(other))]
    Other(Box<str>),
}

#[cfg(not(feature = "serde"))]
impl core::str::FromStr for ProxyType {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VPN" => Ok(Self::VPN),
            "TOR" => Ok(Self::TOR),
            "DCH" => Ok(Self::DCH),
            "PUB" => Ok(Self::PUB),
            "WEB" => Ok(Self::WEB),
            "SES" => Ok(Self::SES),
            "RES" => Ok(Self::RES),
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
        assert_eq!("PUB".parse::<ProxyType>().unwrap(), ProxyType::PUB);
    }
}
