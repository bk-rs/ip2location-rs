//! [Ref](https://lite.ip2location.com/database/px2-ip-proxytype-country#proxy-type)

//
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(
        serde_enum_str::Deserialize_enum_str,
        serde_enum_str::Serialize_enum_str
    )
)]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
pub enum ProxyType {
    VPN,
    TOR,
    DCH,
    PUB,
    WEB,
    SES,
    RES,
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
            s => Ok(Self::Other(s.into())),
        }
    }
}

#[cfg(not(feature = "serde"))]
impl core::fmt::Display for ProxyType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::VPN => write!(f, "VPN"),
            Self::TOR => write!(f, "TOR"),
            Self::DCH => write!(f, "DCH"),
            Self::PUB => write!(f, "PUB"),
            Self::WEB => write!(f, "WEB"),
            Self::SES => write!(f, "SES"),
            Self::RES => write!(f, "RES"),
            Self::Other(s) => write!(f, "{}", s),
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
