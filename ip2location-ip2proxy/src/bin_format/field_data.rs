use core::str;
use std::{io::SeekFrom, net::IpAddr};

use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt as _, AsyncSeekExt as _},
};

use crate::{
    bin_format::{database::LookupError, field::Field, header::Header},
    record::Record,
};

//
#[derive(Debug)]
pub struct FieldData {
    file: TokioFile,
    fields: Vec<Field>,
    buf: Vec<u8>,
}

impl FieldData {
    pub fn new(file: TokioFile, header: Header) -> Self {
        Self {
            file,
            fields: header.r#type.fields()[1..].to_owned(),
            buf: {
                let len = 1 + 255;
                let mut buf = Vec::with_capacity(len);
                buf.resize_with(len, Default::default);
                buf
            },
        }
    }

    pub async fn make(
        &mut self,
        ip_from: IpAddr,
        ip_to: IpAddr,
        indexes: Vec<(Field, u32)>,
    ) -> Result<Record, LookupError> {
        debug_assert_eq!(
            indexes.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
            self.fields
        );

        let mut record = Record::with_empty(ip_from, ip_to);

        for (field, index) in indexes {
            // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L416

            self.file
                .seek(SeekFrom::Start(index as u64))
                .await
                .map_err(LookupError::FileSeekFailed)?;

            //
            let mut n_read = 0;

            //
            let n = self
                .file
                .read(&mut self.buf[..64])
                .await
                .map_err(LookupError::FileReadFailed)?;
            n_read += n;
            if n == 0 {
                return Err(LookupError::Other("read is not completed"));
            }

            //
            let mut n_loop = 0;
            loop {
                loop {
                    if !self.buf.is_empty() {
                        let len = self.buf[0];

                        #[allow(clippy::int_plus_one)]
                        if (len as usize) <= n_read - 1 {
                            break;
                        }
                    }

                    let n = self
                        .file
                        .read(&mut self.buf[n_read..])
                        .await
                        .map_err(LookupError::FileReadFailed)?;
                    n_read += n;

                    if n == 0 {
                        return Err(LookupError::Other("read is not completed 2"));
                    }
                }

                let len = self.buf[0];
                let value = str::from_utf8(&self.buf[1..1 + len as usize]).unwrap();

                match field {
                    Field::IP => {
                        unreachable!()
                    }
                    Field::COUNTRY => {
                        match n_loop {
                            0 => {
                                // values: "-"
                                record.country_code = value.parse().unwrap();

                                n_loop += 1;
                                // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L252
                                // Not 1 + len
                                self.buf.rotate_left(3);

                                continue;
                            }
                            1 => {
                                record.country_name = Some(value.parse().unwrap());

                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                    Field::PROXYTYPE => {
                        record.proxy_type = Some(value.parse().unwrap());
                        break;
                    }
                    Field::REGION => {
                        record.region_name = Some(value.parse().unwrap());
                        break;
                    }
                    Field::CITY => {
                        record.city_name = Some(value.parse().unwrap());
                        break;
                    }
                    Field::ISP => {
                        record.isp = Some(value.parse().unwrap());
                        break;
                    }
                    Field::DOMAIN => {
                        record.domain = Some(value.parse().unwrap());
                        break;
                    }
                    Field::USAGETYPE => {
                        record.usage_type = Some(value.parse().unwrap());
                        break;
                    }
                    Field::ASN => {
                        record.asn = Some(value.parse().unwrap());

                        break;
                    }
                    Field::AS => {
                        record.as_name = Some(value.parse().unwrap());
                        break;
                    }
                    Field::LASTSEEN => {
                        record.last_seen = Some(value.parse().unwrap());

                        break;
                    }
                    Field::THREAT => {
                        record.threat = Some(value.parse().unwrap());
                        break;
                    }
                    Field::PROVIDER => {
                        record.provider = Some(value.parse().unwrap());
                        break;
                    }
                    Field::RESIDENTIAL => {
                        // TODO,
                        break;
                    }
                }
            }
        }

        Ok(record)
    }
}
