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

        let mut field_contents: Vec<(Field, Box<str>)> = vec![];

        for (field, index) in indexes {
            // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L416
            self.file
                .seek(SeekFrom::Start(index as u64))
                .await
                .map_err(LookupError::FileSeekFailed)?;

            let mut n_read = 0;
            let n = self
                .file
                .read(&mut self.buf[..64])
                .await
                .map_err(LookupError::FileReadFailed)?;
            n_read += n;

            if n == 0 {
                return Err(LookupError::Other("read is not completed"));
            }

            let len = self.buf[0];

            loop {
                #[allow(clippy::int_plus_one)]
                if (len as usize) <= n_read - 1 {
                    break;
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

            field_contents.push((
                field,
                str::from_utf8(&self.buf[1..1 + len as usize])
                    .unwrap()
                    .into(),
            ));

            // TODO, https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L252
            // maybe indexes require Field
        }

        let mut record = Record::with_empty(ip_from, ip_to);

        for (field, value) in field_contents {
            match field {
                Field::IP => {}
                Field::COUNTRY => {
                    // values: "-"
                    record.country_code = value;
                }
                Field::PROXYTYPE => {
                    record.proxy_type = Some(value.parse().unwrap());
                }
                Field::REGION => {
                    // TODO,
                }
                Field::CITY => {
                    // TODO,
                }
                Field::ISP => {
                    // TODO,
                }
                Field::DOMAIN => {
                    // TODO,
                }
                Field::USAGETYPE => {
                    // TODO,
                }
                Field::ASN => {
                    // TODO,
                }
                Field::LASTSEEN => {
                    // TODO,
                }
                Field::THREAT => {
                    // TODO,
                }
                Field::RESIDENTIAL => {
                    // TODO,
                }
                Field::PROVIDER => {
                    // TODO,
                }
            }
        }

        Ok(record)
    }
}
