use std::{error, fs::File};

use csv::{ReaderBuilder, StringRecord};
use ip2location_ip2proxy::{
    bin_format::database::Database,
    record::{Record, PX11_CSV_HEADER},
};

#[tokio::test]
async fn test_csv_and_bin() -> Result<(), Box<dyn error::Error>> {
    let path_csv = "data/lite/20220401/IP2PROXY-LITE-PX11.CSV";
    let path_bin = "data/lite/20220401/IP2PROXY-LITE-PX11.BIN";

    //
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path_csv)?);
    let iter = rdr.records();
    let header = StringRecord::from(PX11_CSV_HEADER);

    //
    let mut db = Database::from_file(path_bin).await?;

    //
    for (i, record) in iter.enumerate() {
        if i % 1000 == 0 {
            println!("i:{}", i)
        }

        if rand::random::<u8>() > 25 {
            continue;
        }

        let record = record?;
        let row: Record = record.deserialize(Some(&header))?;

        let record = db.lookup(row.ip_from).await?.unwrap();

        assert!(record.ip_from >= row.ip_from);
        assert!(record.ip_from <= row.ip_to);
        assert_eq!(record.proxy_type, row.proxy_type);
        assert_eq!(record.country_code, row.country_code);
        assert_eq!(record.country_name, row.country_name);
        assert_eq!(record.region_name, row.region_name);
        assert_eq!(record.city_name, row.city_name);
        assert_eq!(record.isp, row.isp);
        assert_eq!(record.domain, row.domain);
        assert_eq!(record.usage_type, row.usage_type);
        assert_eq!(record.asn, row.asn);
        assert_eq!(record.as_name, row.as_name);
        assert_eq!(record.last_seen, row.last_seen);
        assert_eq!(record.threat, row.threat);
        assert_eq!(record.provider, row.provider);
        assert_eq!(record.residential, row.residential);
    }

    Ok(())
}
