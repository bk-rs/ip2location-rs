//
#[derive(Debug, Clone, Copy)]
pub struct Header {
    r#type: Type,
    num_fields: u8,
    year: u8,
    month: u8,
    day: u8,
    num_ipv4_records: u32,
    addr_ipv4: u32,
    num_ipv6_records: u32,
    addr_ipv6: u32,
    index_base_addr_ipv4: u32,
    index_base_addr_ipv6: u32,
    product_code: u8,
    license_code: u8,
    size: u32,
}

//
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Type {
    PX1·LITE,
    PX2·LITE,
    PX3·LITE,
    PX4·LITE,
    PX5·LITE,
    PX6·LITE,
    PX7·LITE,
    PX8·LITE,
    PX9·LITE,
    PX10·LITE,
    PX11·LITE,
}
