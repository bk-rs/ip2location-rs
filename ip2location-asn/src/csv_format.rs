//
pub const CSV_HEADER_DB: &[&str] = &["ip_from", "ip_to", "cidr", "asn", "as"];

#[cfg(feature = "datafusion")]
pub mod datafusion {

    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use arrow_schema::{DataType, Field, Schema};
    use datafusion::{
        common::cast::{as_decimal128_array, as_string_array, as_uint32_array},
        dataframe::DataFrame,
        datasource::file_format::options::CsvReadOptions,
        error::DataFusionError,
        execution::context::SessionContext,
        prelude::{col, lit},
        scalar::ScalarValue,
    };
    use ipnetwork::IpNetwork;

    use crate::record::Record;

    #[derive(Debug, Clone)]
    #[non_exhaustive]
    pub struct Database {
        pub data_frame_v4: DataFrame,
        pub data_frame_v6: DataFrame,
    }

    impl Database {
        pub async fn new(
            csv_path_v4: impl AsRef<str>,
            csv_path_v6: impl AsRef<str>,
        ) -> Result<Self, DataFusionError> {
            let ctx_v4 = SessionContext::new();
            let ctx_v6 = SessionContext::new();

            let schema_v4 = Schema::new(vec![
                Field::new("ip_from", DataType::UInt32, false),
                Field::new("ip_to", DataType::UInt32, false),
                Field::new("cidr", DataType::Utf8, false),
                Field::new("asn", DataType::UInt32, false),
                Field::new("as", DataType::Utf8, false),
            ]);
            let schema_v6 = Schema::new(vec![
                Field::new("ip_from", DataType::Decimal128(39, 0), false),
                Field::new("ip_to", DataType::Decimal128(39, 0), false),
                Field::new("cidr", DataType::Utf8, false),
                Field::new("asn", DataType::UInt32, false),
                Field::new("as", DataType::Utf8, false),
            ]);

            let csv_read_options_v4 = CsvReadOptions::new().has_header(false).schema(&schema_v4);

            let csv_read_options_v6 = CsvReadOptions::new().has_header(false).schema(&schema_v6);

            let data_frame_v4 = ctx_v4
                .read_csv(csv_path_v4.as_ref(), csv_read_options_v4)
                .await?;
            let data_frame_v6 = ctx_v6
                .read_csv(csv_path_v6.as_ref(), csv_read_options_v6)
                .await?;

            Ok(Self {
                data_frame_v4,
                data_frame_v6,
            })
        }
    }

    impl Database {
        pub async fn lookup(&self, ip: IpAddr) -> Result<Option<Record>, DatabaseLookupError> {
            match ip {
                IpAddr::V4(ip) => self.lookup_ipv4(ip).await,
                IpAddr::V6(ip) => self.lookup_ipv6(ip).await,
            }
        }

        pub async fn lookup_ipv4(
            &self,
            ip: Ipv4Addr,
        ) -> Result<Option<Record>, DatabaseLookupError> {
            let ip_u32 = u32::from(ip);

            let df = self
                .data_frame_v4
                .to_owned()
                .filter(
                    col("ip_from")
                        .lt_eq(lit(ip_u32))
                        .and(col("ip_to").gt_eq(lit(ip_u32))),
                )
                .map_err(DatabaseLookupError::DataFusionError)?
                .limit(0, Some(1))
                .map_err(DatabaseLookupError::DataFusionError)?;

            let batches = df
                .collect()
                .await
                .map_err(DatabaseLookupError::DataFusionError)?;

            if let Some(batch) = batches.first() {
                Ok(Some(Record {
                    ip_from: IpAddr::V4(Ipv4Addr::from(
                        as_uint32_array(batch.column_by_name("ip_from").ok_or(
                            DatabaseLookupError::ToRecordFailed("ip_from missing".into()),
                        )?)
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("ip_from invalid, err:{err}").into(),
                            )
                        })?
                        .value(0),
                    )),
                    ip_to: IpAddr::V4(Ipv4Addr::from(
                        as_uint32_array(
                            batch.column_by_name("ip_to").ok_or(
                                DatabaseLookupError::ToRecordFailed("ip_to missing".into()),
                            )?,
                        )
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("ip_to invalid, err:{err}").into(),
                            )
                        })?
                        .value(0),
                    )),
                    cidr: {
                        as_string_array(
                            batch.column_by_name("cidr").ok_or(
                                DatabaseLookupError::ToRecordFailed("cidr missing".into()),
                            )?,
                        )
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("cidr invalid, err:{err}").into(),
                            )
                        })?
                        .value(0)
                        .parse::<IpNetwork>()
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("cidr invalid, err:{err}").into(),
                            )
                        })?
                    },
                    asn: {
                        let v =
                            as_string_array(batch.column_by_name("asn").ok_or(
                                DatabaseLookupError::ToRecordFailed("asn missing".into()),
                            )?)
                            .map_err(|err| {
                                DatabaseLookupError::ToRecordFailed(
                                    format!("asn invalid, err:{err}").into(),
                                )
                            })?
                            .value(0);
                        if v == "-" {
                            None
                        } else {
                            Some(v.parse::<u32>().map_err(|err| {
                                DatabaseLookupError::ToRecordFailed(
                                    format!("asn invalid, err:{err}").into(),
                                )
                            })?)
                        }
                    },
                    r#as: {
                        let v = as_string_array(
                            batch
                                .column_by_name("as")
                                .ok_or(DatabaseLookupError::ToRecordFailed("as missing".into()))?,
                        )
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("as invalid, err:{err}").into(),
                            )
                        })?
                        .value(0);

                        if v == "-" {
                            None
                        } else {
                            Some(v.into())
                        }
                    },
                }))
            } else {
                Ok(None)
            }
        }

        pub async fn lookup_ipv6(
            &self,
            ip: Ipv6Addr,
        ) -> Result<Option<Record>, DatabaseLookupError> {
            let ip_u128 = u128::from(ip);

            let df = self
                .data_frame_v4
                .to_owned()
                .filter(
                    col("ip_from")
                        .lt_eq(lit(ScalarValue::Decimal128(Some(ip_u128 as i128), 39, 0)))
                        .and(col("ip_to").gt_eq(lit(ScalarValue::Decimal128(
                            Some(ip_u128 as i128),
                            39,
                            0,
                        )))),
                )
                .map_err(DatabaseLookupError::DataFusionError)?
                .limit(0, Some(1))
                .map_err(DatabaseLookupError::DataFusionError)?;

            let batches = df
                .collect()
                .await
                .map_err(DatabaseLookupError::DataFusionError)?;

            if let Some(batch) = batches.first() {
                Ok(Some(Record {
                    ip_from: IpAddr::V6(Ipv6Addr::from(
                        as_decimal128_array(batch.column_by_name("ip_from").ok_or(
                            DatabaseLookupError::ToRecordFailed("ip_from missing".into()),
                        )?)
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("ip_from invalid, err:{err}").into(),
                            )
                        })?
                        .value(0) as u128,
                    )),
                    ip_to: IpAddr::V6(Ipv6Addr::from(
                        as_decimal128_array(
                            batch.column_by_name("ip_to").ok_or(
                                DatabaseLookupError::ToRecordFailed("ip_to missing".into()),
                            )?,
                        )
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("ip_to invalid, err:{err}").into(),
                            )
                        })?
                        .value(0) as u128,
                    )),
                    cidr: {
                        as_string_array(
                            batch.column_by_name("cidr").ok_or(
                                DatabaseLookupError::ToRecordFailed("cidr missing".into()),
                            )?,
                        )
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("cidr invalid, err:{err}").into(),
                            )
                        })?
                        .value(0)
                        .parse::<IpNetwork>()
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("cidr invalid, err:{err}").into(),
                            )
                        })?
                    },
                    asn: {
                        let v =
                            as_string_array(batch.column_by_name("asn").ok_or(
                                DatabaseLookupError::ToRecordFailed("asn missing".into()),
                            )?)
                            .map_err(|err| {
                                DatabaseLookupError::ToRecordFailed(
                                    format!("asn invalid, err:{err}").into(),
                                )
                            })?
                            .value(0);
                        if v == "-" {
                            None
                        } else {
                            Some(v.parse::<u32>().map_err(|err| {
                                DatabaseLookupError::ToRecordFailed(
                                    format!("asn invalid, err:{err}").into(),
                                )
                            })?)
                        }
                    },
                    r#as: {
                        let v = as_string_array(
                            batch
                                .column_by_name("as")
                                .ok_or(DatabaseLookupError::ToRecordFailed("as missing".into()))?,
                        )
                        .map_err(|err| {
                            DatabaseLookupError::ToRecordFailed(
                                format!("as invalid, err:{err}").into(),
                            )
                        })?
                        .value(0);

                        if v == "-" {
                            None
                        } else {
                            Some(v.into())
                        }
                    },
                }))
            } else {
                Ok(None)
            }
        }
    }

    //
    #[derive(Debug)]
    pub enum DatabaseLookupError {
        DataFusionError(DataFusionError),
        ToRecordFailed(Box<str>),
    }

    impl core::fmt::Display for DatabaseLookupError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{self:?}")
        }
    }

    impl std::error::Error for DatabaseLookupError {}
}
