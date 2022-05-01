use country_code::{iso3166_2::SubdivisionCode, CountryCode};
use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub country_code: CountryCode,
    pub subdivision_name: Box<str>,
    pub code: SubdivisionCode,
}
