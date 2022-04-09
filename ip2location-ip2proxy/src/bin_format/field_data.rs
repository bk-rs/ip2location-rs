use core::str;
use std::{collections::BTreeMap, io::SeekFrom, net::IpAddr};

use lru::LruCache;
use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt as _, AsyncSeekExt as _},
};

use crate::{
    bin_format::{database::LookupError, field::Field, header::Header},
    proxy_type::ProxyType,
    record::Record,
    usage_type::UsageType,
};

//
pub const COUNTRY_NAME_INDEX_OFFSET: usize = 3;

//
#[derive(Debug)]
pub struct FieldData {
    file: TokioFile,
    fields: Vec<Field>,
    buf: Vec<u8>,
    //
    country_cache: BTreeMap<u32, Box<str>>,
    proxy_type_cache: BTreeMap<u32, ProxyType>,
    usage_type_cache: BTreeMap<u32, UsageType>,
    lru_cache: LruCache<u32, Box<str>>,
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
            country_cache: Default::default(),
            proxy_type_cache: Default::default(),
            usage_type_cache: Default::default(),
            lru_cache: LruCache::new(10000),
        }
    }

    pub async fn make(
        &mut self,
        ip_from: IpAddr,
        ip_to: IpAddr,
        indexes: Vec<(Field, u32)>,
    ) -> Result<Record, LookupError> {
        // TODO, check fields in

        let mut record = Record::with_empty(ip_from, ip_to);

        for (field, index) in indexes {
            match field {
                Field::IP => {
                    unreachable!()
                }
                Field::COUNTRY => {
                    if let Some(value) = self.country_cache.get(&index) {
                        record.country_code = value.parse().unwrap();

                        if let Some(value) = self
                            .country_cache
                            .get(&(index + COUNTRY_NAME_INDEX_OFFSET as u32))
                        {
                            record.country_name = Some(value.parse().unwrap());

                            continue;
                        }
                    }
                }
                Field::PROXYTYPE => {
                    if let Some(value) = self.proxy_type_cache.get(&index) {
                        record.proxy_type = Some(value.to_owned());

                        continue;
                    }
                }
                Field::REGION => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.region_name = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::CITY => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.city_name = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::ISP => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.isp = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::DOMAIN => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.domain = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::USAGETYPE => {
                    if let Some(value) = self.usage_type_cache.get(&index) {
                        record.usage_type = Some(value.to_owned());

                        continue;
                    }
                }
                Field::ASN => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.asn = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::AS => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.as_name = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::LASTSEEN => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.last_seen = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::THREAT => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.threat = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::PROVIDER => {
                    if let Some(value) = self.lru_cache.get(&index) {
                        record.provider = Some(value.parse().unwrap());

                        continue;
                    }
                }
                Field::RESIDENTIAL => {}
            }

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
                let value: Box<str> = str::from_utf8(&self.buf[1..1 + len as usize])
                    .unwrap()
                    .into();

                match field {
                    Field::IP => {
                        unreachable!()
                    }
                    Field::COUNTRY => {
                        match n_loop {
                            0 => {
                                record.country_code = value.parse().unwrap();

                                self.country_cache.insert(index, value);

                                n_loop += 1;
                                // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L252
                                // Not 1 + len
                                self.buf.rotate_left(COUNTRY_NAME_INDEX_OFFSET);

                                continue;
                            }
                            1 => {
                                record.country_name = Some(value.parse().unwrap());

                                self.country_cache
                                    .insert(index + COUNTRY_NAME_INDEX_OFFSET as u32, value);

                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                    Field::PROXYTYPE => {
                        record.proxy_type = Some(value.parse().unwrap());

                        self.proxy_type_cache.insert(index, value.parse().unwrap());

                        break;
                    }
                    Field::REGION => {
                        record.region_name = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

                        break;
                    }
                    Field::CITY => {
                        record.city_name = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

                        break;
                    }
                    Field::ISP => {
                        record.isp = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

                        break;
                    }
                    Field::DOMAIN => {
                        record.domain = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

                        break;
                    }
                    Field::USAGETYPE => {
                        record.usage_type = Some(value.parse().unwrap());

                        self.usage_type_cache.insert(index, value.parse().unwrap());

                        break;
                    }
                    Field::ASN => {
                        record.asn = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

                        break;
                    }
                    Field::AS => {
                        record.as_name = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

                        break;
                    }
                    Field::LASTSEEN => {
                        record.last_seen = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

                        break;
                    }
                    Field::THREAT => {
                        record.threat = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

                        break;
                    }
                    Field::PROVIDER => {
                        record.provider = Some(value.parse().unwrap());

                        self.lru_cache.put(index, value);

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
