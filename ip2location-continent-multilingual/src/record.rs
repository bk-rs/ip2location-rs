use continent_code::ContinentCode;
use country_code::CountryCode;
use language_code::LanguageTag;
use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub lang: LanguageTag,
    pub country_alpha2_code: CountryCode,
    pub continent_code: ContinentCode,
    pub continent: Box<str>,
}
