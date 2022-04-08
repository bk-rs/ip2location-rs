use std::net::Ipv6Addr;

use tokio::fs::File as TokioFile;

use crate::{
    bin_format::{
        database::LookupError,
        header::{Header, Type},
        ipv6_index::Ipv6Index,
    },
    record::Record,
};

//
#[derive(Debug)]
pub struct Ipv6Data {
    file: TokioFile,
    index: Ipv6Index,
    info: Ipv6DataInfo,
    r#type: Type,
}

impl Ipv6Data {
    pub fn new(file: TokioFile, index: Ipv6Index, header: Header) -> Self {
        Self {
            file,
            index,
            info: header.ipv6_data_info,
            r#type: header.r#type,
        }
    }
    pub async fn lookup(&mut self, addr: Ipv6Addr) -> Result<Option<Record>, LookupError> {
        todo!()
    }
}

//
//
//
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct Ipv6DataInfo {
    // >= 0
    pub count: u32,
    // > 0
    pub index_start: u32,
}

impl Ipv6DataInfo {
    pub fn size(&self, num_fields: u8) -> u32 {
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L231
        self.count * ((num_fields as u32) * 4 + 12)
    }

    pub fn index_end(&self, num_fields: u8) -> u32 {
        self.index_start + self.size(num_fields)
    }
}
