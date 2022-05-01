#![cfg(feature = "once_cell")]

use country_code::CountryCode;
use currency_code::CurrencyCode;
use ip2location_country_information::RECORDS_COUNTRY_CODE_MAP;

#[test]
fn test_static() {
    //
    let record = RECORDS_COUNTRY_CODE_MAP.get(&CountryCode::US).unwrap();
    println!("{:?}", record);
    assert_eq!(record.country_name, "United States of America".into());

    //
    let record = RECORDS_COUNTRY_CODE_MAP.get(&CountryCode::AQ).unwrap();
    println!("{:?}", record);
    assert_eq!(record.currency_code, CurrencyCode::Other("".into()));
}
