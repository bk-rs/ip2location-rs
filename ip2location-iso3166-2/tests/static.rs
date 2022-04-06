#![cfg(feature = "once_cell")]

use ip2location_iso3166_2::DATA_MAP;

#[test]
fn test_static() {
    let row = DATA_MAP.get("US-NY").unwrap();
    assert_eq!(row.subdivision_name, "New York".into());
}
