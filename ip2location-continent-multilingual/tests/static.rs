#![cfg(feature = "once_cell")]

use country_code::CountryCode;
use ip2location_continent_multilingual::RECORDS;
use language_code::{LanguageCode, LanguageTag};

#[test]
fn test_static() {
    //
    let record = RECORDS
        .iter()
        .find(|x| {
            x.lang == LanguageTag::new(LanguageCode::en, None)
                && x.country_alpha2_code == CountryCode::US
        })
        .unwrap();
    println!("{record:?}");

    //
    let record = RECORDS
        .iter()
        .find(|x| {
            x.lang == LanguageTag::new(LanguageCode::zh, Some(CountryCode::CN))
                && x.country_alpha2_code == CountryCode::US
        })
        .unwrap();
    println!("{record:?}");
}
