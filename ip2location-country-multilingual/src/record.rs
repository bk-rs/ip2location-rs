use country_code::{iso3166_1::alpha_3::CountryCode as CountryCode3, CountryCode};
use language_code::LanguageTag;
use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub lang: LanguageTag,
    pub lang_name: Box<str>,
    pub country_alpha2_code: CountryCode,
    pub country_alpha3_code: CountryCode3,
    pub country_numeric_code: Box<str>,
    pub country_name: Box<str>,
}
