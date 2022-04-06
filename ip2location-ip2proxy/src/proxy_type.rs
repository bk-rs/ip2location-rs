//! [Ref](https://lite.ip2location.com/database/px2-ip-proxytype-country#proxy-type)

use serde_enum_str::Deserialize_enum_str;

//
#[derive(Deserialize_enum_str, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProxyType {
    VPN,
    TOR,
    DCH,
    PUB,
    WEB,
    SES,
    RES,
    #[serde(other)]
    Other(Box<str>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!("PUB".parse::<ProxyType>().unwrap(), ProxyType::PUB);
    }
}
