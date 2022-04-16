#![cfg(feature = "once_cell")]

use ip2location_continent_multilingual::RECORDS;

#[test]
fn test_static() {
    let record = RECORDS
        .iter()
        .find(|x| x.lang == "EN".into() && x.country_alpha2_code == "US".into())
        .unwrap();
    println!("{:?}", record);
}
