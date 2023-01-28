use core::{fmt, ops::Deref};
use std::io::Read;

use csv::{Error as CsvError, Reader};

use crate::record::Record;

//
#[cfg(feature = "once_cell")]
pub static RECORDS: once_cell::sync::Lazy<Records> = once_cell::sync::Lazy::new(|| {
    let csv = include_str!("../data/IP2LOCATION-CONTINENT-MULTILINGUAL.CSV");
    Records::from_csv(csv.as_bytes()).unwrap()
});

//
#[derive(Debug, Clone)]
pub struct Records(pub Vec<Record>);

impl Deref for Records {
    type Target = Vec<Record>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//
impl Records {
    pub fn from_csv<R: Read>(rdr: R) -> Result<Self, RecordsFromCsvError> {
        let mut rdr = Reader::from_reader(rdr);

        let mut inner = vec![];

        for record in rdr.records() {
            let record = record.map_err(RecordsFromCsvError::CsvParseFailed)?;
            let row: Record = record
                .deserialize(None)
                .map_err(RecordsFromCsvError::RecordDeFailed)?;
            inner.push(row);
        }

        Ok(Self(inner))
    }
}

//
#[derive(Debug)]
pub enum RecordsFromCsvError {
    CsvParseFailed(CsvError),
    RecordDeFailed(CsvError),
}

impl fmt::Display for RecordsFromCsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RecordsFromCsvError {}
