use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub country_code: Box<str>,
    pub subdivision_name: Box<str>,
    pub code: Box<str>,
}
