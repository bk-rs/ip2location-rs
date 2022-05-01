use country_code::{iso3166_1::alpha_3::CountryCode as CountryCode3, CountryCode};
use currency_code::CurrencyCode;
use language_code::LanguageCode;
use serde::Deserialize;

//
#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    pub country_code: CountryCode,
    pub country_name: Box<str>,
    pub country_alpha3_code: CountryCode3,
    pub country_numeric_code: Box<str>,
    pub capital: Box<str>,
    pub country_demonym: Box<str>,
    pub total_area: f64,
    pub population: u32,
    pub idd_code: Box<str>,
    #[serde(default, deserialize_with = "currency_code_deserialize")]
    pub currency_code: CurrencyCode,
    pub currency_name: Box<str>,
    pub currency_symbol: Box<str>,
    pub lang_code: LanguageCode,
    pub lang_name: Box<str>,
    pub cctld: Box<str>,
}

//
fn currency_code_deserialize<'de, D>(deserializer: D) -> Result<CurrencyCode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;

    let s = Box::<str>::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(CurrencyCode::Other("".into()))
    } else {
        Ok(s.parse::<CurrencyCode>()
            .map_err(serde::de::Error::custom)?)
    }
}
