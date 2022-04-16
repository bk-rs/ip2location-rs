use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub lang: Box<str>,
    pub lang_name: Box<str>,
    pub country_alpha2_code: Box<str>,
    pub country_alpha3_code: Box<str>,
    pub country_numeric_code: Box<str>,
    pub country_name: Box<str>,
}
