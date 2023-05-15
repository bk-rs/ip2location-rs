use std::{fs::File, net::IpAddr};

use csv::{ReaderBuilder, StringRecord};
use ip2location_asn::{
    csv_format::{datafusion::Database, CSV_HEADER_DB},
    record::Record,
};

#[tokio::test]
async fn test_db() -> Result<(), Box<dyn std::error::Error>> {
    let path_csv_v4 = "data/asn-lite/latest/IP2LOCATION-LITE-ASN.CSV";
    let path_csv_v6 = "data/asn-lite/latest/IP2LOCATION-LITE-ASN.IPV6.CSV";

    //
    let mut csv_rdr_v4 = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path_csv_v4)?);

    let mut csv_rdr_v6 = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path_csv_v6)?);

    let header = StringRecord::from(CSV_HEADER_DB);

    //
    let db = Database::new(path_csv_v4, path_csv_v6).await?;

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

        let record = match db.lookup(row.ip_from).await? {
            Some(x) => x,
            None => match row.ip_from {
                IpAddr::V4(ip) => {
                    panic!("v4 {:?}", u32::from(ip));
                }
                IpAddr::V6(_) => {
                    unreachable!()
                }
            },
        };
        count_v4 += 1;

        assert!(record.ip_from >= row.ip_from);
        assert!(record.ip_from <= row.ip_to);
        assert_eq!(record.cidr, row.cidr);
        assert_eq!(record.asn, row.asn);
        assert_eq!(record.r#as, row.r#as);
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

        let record = match db.lookup(row.ip_from).await? {
            Some(x) => x,
            None => match row.ip_from {
                IpAddr::V4(_) => {
                    unreachable!()
                }
                IpAddr::V6(ip) => {
                    panic!("v6 {:?}", u128::from(ip));
                }
            },
        };
        count_v6 += 1;

        assert!(record.ip_from >= row.ip_from);
        assert!(record.ip_from <= row.ip_to);
        assert_eq!(record.cidr, row.cidr);
        assert_eq!(record.asn, row.asn);
        assert_eq!(record.r#as, row.r#as);
    }
    println!("count_v6:{count_v6}");

    Ok(())
}
