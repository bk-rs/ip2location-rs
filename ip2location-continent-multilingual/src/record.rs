use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub lang: Box<str>,
    pub country_alpha2_code: Box<str>,
    pub continent_code: Box<str>,
    pub continent: Box<str>,
}
