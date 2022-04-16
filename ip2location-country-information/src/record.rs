use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub country_code: Box<str>,
    pub country_name: Box<str>,
    pub country_alpha3_code: Box<str>,
    pub country_numeric_code: Box<str>,
    pub capital: Box<str>,
    pub country_demonym: Box<str>,
    pub total_area: f64,
    pub population: u32,
    pub idd_code: Box<str>,
    pub currency_code: Box<str>,
    pub currency_name: Box<str>,
    pub currency_symbol: Box<str>,
    pub lang_code: Box<str>,
    pub lang_name: Box<str>,
    pub cctld: Box<str>,
}
