use std::{error, fs::File};

use csv::{ReaderBuilder, StringRecord};
use ip2location_ip2proxy::{
    bin_format::Database, csv_format::CSV_HEADER_PX11, proxy_type::ProxyType, record::Record,
};

#[tokio::test]
async fn test_px11() -> Result<(), Box<dyn error::Error>> {
    let path_csv_v4 = "data/ip2proxy-lite/20220401/IP2PROXY-LITE-PX11.CSV";
    let path_csv_v6 = "data/ip2proxy-lite/20220401/IP2PROXY-LITE-PX11.IPV6.CSV";
    let path_bin = "data/ip2proxy-lite/20220401/IP2PROXY-LITE-PX11.BIN";

    //
    let mut rdr_v4 = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path_csv_v4)?);
    let iter_v4 = rdr_v4.records();

    let mut rdr_v6 = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path_csv_v6)?);
    let iter_v6 = rdr_v6.records();

    let header = StringRecord::from(CSV_HEADER_PX11);

    //
    let mut db = Database::<async_compat::Compat<tokio::fs::File>>::new(path_bin).await?;
    println!("{:?}", db.inner.header);

    //
    let mut count_v4 = 0;
    for (i, record) in iter_v4.enumerate() {
        if i % 100000 == 0 {
            println!("v4 i:{}", i)
        }

        if rand::random::<u8>() > 2 {
            continue;
        }

        let record = record?;
        let row: Record = record.deserialize(Some(&header))?;

        let record = db.lookup(row.ip_from, None).await?.unwrap();
        count_v4 += 1;

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
    println!("count_v4:{}", count_v4);

    //
    let mut count_v6 = 0;
    for (i, record) in iter_v6.enumerate() {
        if i % 100000 == 0 {
            println!("v6 i:{}", i)
        }

        if rand::random::<u8>() > 2 {
            continue;
        }

        let record = record?;
        let row: Record = record.deserialize(Some(&header))?;

        match db.lookup(row.ip_from, None).await? {
            Some(record) => {
                if record.proxy_type.is_some() && record.proxy_type != Some(ProxyType::Unknown) {
                    count_v6 += 1;

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
            }
            None => {}
        }
    }
    println!("count_v6:{}", count_v6);

    Ok(())
}
