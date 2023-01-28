use std::{error, fs::File, net::IpAddr};

use csv::{ReaderBuilder, StringRecord};
use ip2location_ip2location::{
    bin_format::{Database, TokioFile},
    csv_format::CSV_HEADER_DB11,
    record::Record,
};

#[tokio::test]
async fn test_db11() -> Result<(), Box<dyn error::Error>> {
    let path_csv_v4 = "data/ip2location-lite/20221101/IP2LOCATION-LITE-DB11.CSV";
    let path_csv_v6 = "data/ip2location-lite/20221101/IP2LOCATION-LITE-DB11.IPV6.CSV";
    let path_bin_v4 = "data/ip2location-lite/20221101/IP2LOCATION-LITE-DB11.BIN";
    let path_bin_v6 = "data/ip2location-lite/20221101/IP2LOCATION-LITE-DB11.IPV6.BIN";

    //
    let mut csv_rdr_v4 = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path_csv_v4)?);

    let mut csv_rdr_v6 = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path_csv_v6)?);

    let header = StringRecord::from(CSV_HEADER_DB11);

    //
    let db_v4 = Database::<TokioFile>::new(path_bin_v4, 2).await?;
    println!("{:?}", db_v4.inner.header);

    let db_v6 = Database::<TokioFile>::new(path_bin_v6, 2).await?;
    println!("{:?}", db_v6.inner.header);

    //
    let mut count_v4 = 0;
    for (i, record) in csv_rdr_v4.records().enumerate() {
        if i % 100000 == 0 {
            println!("v4 i:{i}")
        }

        if rand::random::<u8>() > 2 {
            continue;
        }

        let record = record?;
        let row: Record = record.deserialize(Some(&header))?;

        let record = match db_v4.lookup(row.ip_from, None).await? {
            Some(x) => x,
            None => {
                if row.country_code.is_default() {
                    continue;
                } else {
                    match row.ip_from {
                        IpAddr::V4(ip) => {
                            panic!("v4 {:?}", u32::from(ip));
                        }
                        IpAddr::V6(_) => {
                            unreachable!()
                        }
                    }
                }
            }
        };
        count_v4 += 1;

        assert!(record.ip_from >= row.ip_from);
        assert!(record.ip_from <= row.ip_to);
        assert_eq!(record.country_code, row.country_code);
        assert_eq!(record.country_name, row.country_name);
        assert_eq!(record.region_name, row.region_name);
        assert_eq!(record.city_name, row.city_name);
        assert_eq!(record.latitude, row.latitude);
        assert_eq!(record.longitude, row.longitude);
        assert_eq!(record.zip_code, row.zip_code);
        assert_eq!(record.time_zone, row.time_zone);
    }
    println!("count_v4:{count_v4}");

    //
    let mut count_v6 = 0;
    for (i, record) in csv_rdr_v6.records().enumerate() {
        if i % 100000 == 0 {
            println!("v6 i:{i}")
        }

        if rand::random::<u8>() > 2 {
            continue;
        }

        let record = record?;
        let row: Record = record.deserialize(Some(&header))?;

        let record = match db_v6.lookup(row.ip_from, None).await? {
            Some(x) => x,
            None => {
                if row.country_code.is_default() {
                    continue;
                } else {
                    match row.ip_from {
                        IpAddr::V4(_) => {
                            unreachable!()
                        }
                        IpAddr::V6(ip) => {
                            panic!("v6 {:?}", u128::from(ip));
                        }
                    }
                }
            }
        };
        count_v6 += 1;

        assert!(record.ip_from >= row.ip_from);
        assert!(record.ip_from <= row.ip_to);
        assert_eq!(record.country_code, row.country_code);
        assert_eq!(record.country_name, row.country_name);
        assert_eq!(record.region_name, row.region_name);
        assert_eq!(record.city_name, row.city_name);
        assert_eq!(record.latitude, row.latitude);
        assert_eq!(record.longitude, row.longitude);
        assert_eq!(record.zip_code, row.zip_code);
        assert_eq!(record.time_zone, row.time_zone);
    }
    println!("count_v6:{count_v6}");

    Ok(())
}
