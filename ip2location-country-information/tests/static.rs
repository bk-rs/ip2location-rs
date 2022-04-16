#![cfg(feature = "once_cell")]

use ip2location_country_information::RECORDS_COUNTRY_CODE_MAP;

#[test]
fn test_static() {
    let record = RECORDS_COUNTRY_CODE_MAP.get("US").unwrap();
    println!("{:?}", record);
    assert_eq!(record.country_alpha3_code, "USA".into());
}
