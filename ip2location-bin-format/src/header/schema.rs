use core::fmt;

use super::HEADER_LEN;
use crate::{index::INDEX_LEN, record_field::RecordFields};

//
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct Schema {
    pub sub_type: SchemaSubType,
    pub num_record_fields: u8,
    pub date: (u8, u8, u8),
    pub v4_records_count: u32,
    pub v4_records_position_start: u32,
    pub v6_records_count: u32,
    pub v6_records_position_start: u32,
    pub v4_index_position_start: u32,
    pub v6_index_position_start: u32,
    pub r#type: SchemaType,
    // TODO, what's meaning?
    pub license_code: u8,
    pub total_size: u32,
}

impl Schema {
    pub fn record_fields(&self) -> Option<RecordFields> {
        RecordFields::try_from((self.r#type, self.sub_type)).ok()
    }

    pub fn has_v6(&self) -> bool {
        self.v6_records_count > 0
    }

    pub fn v4_index_seek_from_start(&self) -> u64 {
        self.v4_index_position_start as u64 - 1
    }

    pub fn v6_index_seek_from_start(&self) -> Option<u64> {
        if self.has_v6() {
            Some(self.v6_index_position_start as u64 - 1)
        } else {
            None
        }
    }

    pub fn v4_records_seek_from_start(&self) -> u64 {
        self.v4_records_position_start as u64 - 1
    }

    pub fn v6_records_seek_from_start(&self) -> Option<u64> {
        if self.has_v6() {
            Some(self.v6_records_position_start as u64 - 1)
        } else {
            None
        }
    }
}

impl Schema {
    pub fn verify(&self) -> Result<(), VerifyError> {
        //
        let record_fields = self
            .record_fields()
            .ok_or(VerifyError::SubTypeInvalid(self.sub_type))?;

        if record_fields.len() != self.num_record_fields as usize {
            return Err(VerifyError::NumRecordFieldsMismatch(self.num_record_fields));
        }

        //
        if !self.has_v6() {
            if self.v6_index_position_start != 1 {
                return Err(VerifyError::Other(
                    "v6_index_position_start should eq 1 when v6_records_count is 0".into(),
                ));
            }

            if self.v6_records_position_start != 1 {
                return Err(VerifyError::Other(
                    "v6_records_position_start should eq 1 when v6_records_count is 0".into(),
                ));
            }
        }

        //
        let mut cur_position: u32 = 0;
        cur_position += HEADER_LEN + 1;

        if self.v4_index_position_start != cur_position {
            return Err(VerifyError::XPositionStartInvalid(
                format!(
                    "v4_index_position_start mismatch {} {}",
                    self.v4_index_position_start, cur_position
                )
                .into(),
            ));
        }
        cur_position += INDEX_LEN;

        if self.has_v6() {
            if self.v6_index_position_start != cur_position {
                return Err(VerifyError::XPositionStartInvalid(
                    format!(
                        "v6_index_position_start mismatch {} {}",
                        self.v6_index_position_start, cur_position
                    )
                    .into(),
                ));
            }
            cur_position += INDEX_LEN;
        }

        if self.v4_records_position_start != cur_position {
            return Err(VerifyError::XPositionStartInvalid(
                format!(
                    "v4_records_position_start mismatch {} {}",
                    self.v4_records_position_start, cur_position
                )
                .into(),
            ));
        }
        cur_position += record_fields.records_bytes_len_for_ipv4(self.v4_records_count);

        if self.has_v6() {
            if self.v6_records_position_start != cur_position {
                return Err(VerifyError::XPositionStartInvalid(
                    format!(
                        "v6_records_position_start mismatch {} {}",
                        self.v6_records_position_start, cur_position
                    )
                    .into(),
                ));
            }
            cur_position += record_fields.records_bytes_len_for_ipv6(self.v6_records_count);
        }

        if cur_position >= self.total_size {
            return Err(VerifyError::TotalSizeTooSmall(self.total_size));
        }

        Ok(())
    }
}

//
#[derive(Debug)]
pub enum VerifyError {
    SubTypeInvalid(SchemaSubType),
    NumRecordFieldsMismatch(u8),
    XPositionStartInvalid(Box<str>),
    TotalSizeTooSmall(u32),
    Other(Box<str>),
}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for VerifyError {}

//
//
//
#[derive(Debug, Clone, Copy, Default)]
pub struct SchemaSubType(pub u8);

//
//
//
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    None,
    IP2Location,
    IP2Proxy,
}

impl Default for SchemaType {
    fn default() -> Self {
        Self::None
    }
}

impl TryFrom<u8> for SchemaType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::IP2Location),
            2 => Ok(Self::IP2Proxy),
            _ => Err(()),
        }
    }
}

impl SchemaType {
    pub fn is_ip2location(&self) -> bool {
        matches!(self, Self::IP2Location | Self::None)
    }

    pub fn is_ip2proxy(&self) -> bool {
        matches!(self, Self::IP2Proxy)
    }
}
