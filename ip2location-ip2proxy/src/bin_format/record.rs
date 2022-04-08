use core::fmt;
use std::net::IpAddr;

use crate::{bin_format::field::Field, record::Record};

//
pub fn parse(
    ip_from: IpAddr,
    ip_to: IpAddr,
    fields_without_ip_slice: &[u8],
    fields_without_ip: &[Field],
) -> Result<Record, ParseError> {
    let record = Record::with_empty(ip_from, ip_to);

    Ok(record)
}

//
#[derive(Debug)]
pub enum ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}
