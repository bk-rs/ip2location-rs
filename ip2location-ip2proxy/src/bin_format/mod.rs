pub mod database;
pub mod field;
pub mod header;
pub mod ipv4_data;
pub mod ipv4_index;
pub mod ipv6_data;
pub mod ipv6_index;

#[cfg(test)]
pub(crate) const TEST_BIN_FILES: &[(&str, self::header::Type)] = &[
    (
        "data/20220401/IP2PROXY-LITE-PX1.BIN",
        self::header::Type::PX1,
    ),
    (
        "data/20220401/IP2PROXY-LITE-PX11.BIN",
        self::header::Type::PX11,
    ),
];

#[cfg(test)]
pub(crate) const TEST_BIN_IPV4_ADDRS: &[&str] = &["1.0.4.1", "1.0.5.1", "1.0.11.253"];
