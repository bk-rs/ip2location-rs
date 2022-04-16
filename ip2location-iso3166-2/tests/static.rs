#![cfg(feature = "once_cell")]

use ip2location_iso3166_2::RECORDS_CODE_MAP;

#[test]
fn test_static() {
    let record = RECORDS_CODE_MAP.get("US-NY").unwrap();
    println!("{:?}", record);
    assert_eq!(record.subdivision_name, "New York".into());
}
