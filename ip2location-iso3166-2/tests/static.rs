#![cfg(feature = "once_cell")]

use country_code::{
    iso3166_2::{CNSubdivisionCode, SubdivisionCode, USSubdivisionCode},
    CountryCode,
};
use ip2location_iso3166_2::RECORDS_CODE_MAP;

#[test]
fn test_static() {
    //
    let record = RECORDS_CODE_MAP
        .get(&SubdivisionCode::US(USSubdivisionCode::NY))
        .unwrap();
    println!("{:?}", record);
    assert_eq!(record.subdivision_name, "New York".into());

    //
    let record = RECORDS_CODE_MAP
        .get(&SubdivisionCode::CN(CNSubdivisionCode::BJ))
        .unwrap();
    println!("{:?}", record);
    assert_eq!(record.subdivision_name, "Beijing".into());

    //
    let record = RECORDS_CODE_MAP
        .get(&SubdivisionCode::Other(CountryCode::AI, None))
        .unwrap();
    println!("{:?}", record);
    assert_eq!(record.subdivision_name, "Anguilla".into());
}
