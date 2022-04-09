pub mod database;
pub mod field;
pub mod field_data;
pub mod header;
pub mod ipv4_data;
pub mod ipv4_index;
pub mod ipv6_data;
pub mod ipv6_index;

#[cfg(test)]
pub(crate) const TEST_LITE_20220401_BIN_FILES: &[&str] = &[
    "data/ip2proxy-lite/20220401/IP2PROXY-LITE-PX1.BIN",
    "data/ip2proxy-lite/20220401/IP2PROXY-LITE-PX2.BIN",
    "data/ip2proxy-lite/20220401/IP2PROXY-LITE-PX11.BIN",
];

#[cfg(test)]
pub(crate) const TEST_LITE_20220401_BIN_IPV4_ADDRS: &[u32] = &[
    16778241, 16778497, 16780285, 3758093800, 3758094066, 3758094285,
];

#[cfg(test)]
pub(crate) const TEST_LITE_20220401_BIN_IPV6_ADDRS: &[u128] = &[
    281470698521601,
    281470698521857,
    281470698523645,
    58569071813452613181225066592045428949,
    58569071813452613184123847728234381123,
    58569071813452613185929873510317667680,
];
