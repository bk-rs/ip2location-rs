use core::{fmt, ops::Deref};
use std::io::Read;

use csv::{Error as CsvError, Reader as CsvReader};
use serde::Deserialize;

//
#[cfg(feature = "once_cell")]
pub static DATA: once_cell::sync::Lazy<Data> = once_cell::sync::Lazy::new(|| {
    let csv = include_str!("../data/IP2LOCATION-ISO3166-2.CSV");
    Data::from_csv(csv.as_bytes()).unwrap()
});

#[cfg(feature = "once_cell")]
pub static DATA_MAP: once_cell::sync::Lazy<std::collections::HashMap<Box<str>, Row>> =
    once_cell::sync::Lazy::new(|| {
        DATA.iter()
            .cloned()
            .map(|x| (x.code.to_owned(), x))
            .collect()
    });

//
#[derive(Debug, Clone)]
pub struct Data(pub Vec<Row>);

impl Deref for Data {
    type Target = Vec<Row>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//
#[derive(Deserialize, Debug, Clone)]
pub struct Row {
    pub country_code: Box<str>,
    pub subdivision_name: Box<str>,
    pub code: Box<str>,
}

//
impl Data {
    pub fn from_csv<R: Read>(rdr: R) -> Result<Self, DataFromCsvError> {
        let mut rdr = CsvReader::from_reader(rdr);
        let iter = rdr.records();

        let mut inner = vec![];

        for record in iter {
            let record = record.map_err(DataFromCsvError::CsvParseFailed)?;
            let row: Row = record
                .deserialize(None)
                .map_err(DataFromCsvError::RowDeFailed)?;
            inner.push(row);
        }

        Ok(Self(inner))
    }
}

//
#[derive(Debug)]
pub enum DataFromCsvError {
    CsvParseFailed(CsvError),
    RowDeFailed(CsvError),
}

impl fmt::Display for DataFromCsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DataFromCsvError {}
