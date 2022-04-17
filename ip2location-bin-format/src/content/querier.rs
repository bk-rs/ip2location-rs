use core::{fmt, str};
use std::{
    collections::BTreeMap,
    io::{Error as IoError, SeekFrom},
};

use futures_util::{AsyncRead, AsyncReadExt as _, AsyncSeek, AsyncSeekExt as _};

use crate::record_field::{RecordFieldContent, RecordFieldContents};

//
pub const COUNTRY_NAME_INDEX_OFFSET: usize = 3;

//
#[derive(Debug)]
pub struct Querier<S> {
    stream: S,
    buf: Vec<u8>,
    static_cache: BTreeMap<u32, Box<str>>,
    #[cfg(feature = "lru")]
    lru_cache: lru::LruCache<u32, Box<str>>,
}

//
//
//
impl<S> Querier<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            buf: {
                let len = 1 + 255;
                let mut buf = Vec::with_capacity(len);
                buf.resize_with(len, Default::default);
                buf
            },
            static_cache: BTreeMap::default(),
            #[cfg(feature = "lru")]
            lru_cache: lru::LruCache::new(10000),
        }
    }
}

//
//
//
impl<S> Querier<S>
where
    S: AsyncSeek + AsyncRead + Unpin,
{
    pub async fn fill(
        &mut self,
        record_field_contents: &mut RecordFieldContents,
    ) -> Result<(), FillError> {
        for record_field_content in record_field_contents.iter_mut() {
            //
            let (seek_from_start, s_len_estimatable) = match record_field_content {
                RecordFieldContent::COUNTRY(i, v, v_name) => {
                    if let Some(s) = self.static_cache.get(i) {
                        *v = s.to_owned();

                        if let Some(s) = self
                            .static_cache
                            .get(&(*i + COUNTRY_NAME_INDEX_OFFSET as u32))
                        {
                            *v_name = s.to_owned();

                            continue;
                        }
                    }

                    (*i, 28)
                }
                #[allow(unused_variables)]
                RecordFieldContent::REGION(i, v) => {
                    #[cfg(feature = "lru")]
                    {
                        if let Some(s) = self.lru_cache.get(i) {
                            *v = s.to_owned();

                            continue;
                        }
                    }

                    (*i, 20)
                }
                #[allow(unused_variables)]
                RecordFieldContent::CITY(i, v) => {
                    #[cfg(feature = "lru")]
                    {
                        if let Some(s) = self.lru_cache.get(i) {
                            *v = s.to_owned();

                            continue;
                        }
                    }

                    (*i, 20)
                }
                RecordFieldContent::ISP(i, _) => (*i, 10),
                RecordFieldContent::DOMAIN(i, _) => (*i, 30),
                //
                RecordFieldContent::LATITUDE(_) => continue,
                RecordFieldContent::LONGITUDE(_) => continue,
                #[allow(unused_variables)]
                RecordFieldContent::ZIPCODE(i, v) => {
                    #[cfg(feature = "lru")]
                    {
                        if let Some(s) = self.lru_cache.get(i) {
                            *v = s.to_owned();

                            continue;
                        }
                    }

                    (*i, 8)
                }
                #[allow(unused_variables)]
                RecordFieldContent::TIMEZONE(i, v) => {
                    #[cfg(feature = "lru")]
                    {
                        if let Some(s) = self.lru_cache.get(i) {
                            *v = s.to_owned();

                            continue;
                        }
                    }

                    (*i, 8)
                }
                RecordFieldContent::NETSPEED(i, v) => {
                    if let Some(s) = self.static_cache.get(i) {
                        *v = s.to_owned();

                        continue;
                    }

                    // TODO, 10
                    (*i, 10)
                }
                //
                RecordFieldContent::PROXYTYPE(i, v) => {
                    if let Some(s) = self.static_cache.get(i) {
                        *v = s.to_owned();

                        continue;
                    }

                    (*i, 3)
                }
                RecordFieldContent::USAGETYPE(i, v) => {
                    if let Some(s) = self.static_cache.get(i) {
                        *v = s.to_owned();

                        continue;
                    }

                    (*i, 3)
                }
                RecordFieldContent::ASN(i, _) => (*i, 10),
                RecordFieldContent::AS(i, _) => (*i, 30),
                RecordFieldContent::LASTSEEN(i, _) => (*i, 6),
                RecordFieldContent::THREAT(i, _) => (*i, 30),
                RecordFieldContent::RESIDENTIAL(i, _) => (*i, 30),
                RecordFieldContent::PROVIDER(i, _) => (*i, 30),
            };

            //
            //
            //
            // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L416
            self.stream
                .seek(SeekFrom::Start(seek_from_start as u64))
                .await
                .map_err(FillError::SeekFailed)?;

            //
            let mut n_read = 0;

            //
            let n = self
                .stream
                .read(&mut self.buf[..s_len_estimatable + 1])
                .await
                .map_err(FillError::ReadFailed)?;
            n_read += n;
            if n == 0 {
                return Err(FillError::Other("read is not completed in first read"));
            }

            //
            let mut n_loop = 0;
            loop {
                //
                loop {
                    if !self.buf.is_empty() {
                        let len = self.buf[0];

                        #[allow(clippy::int_plus_one)]
                        if (len as usize) <= n_read - 1 {
                            break;
                        }
                    }

                    let n = self
                        .stream
                        .read(&mut self.buf[n_read..])
                        .await
                        .map_err(FillError::ReadFailed)?;
                    n_read += n;

                    if n == 0 {
                        return Err(FillError::Other("read is not completed in loop read"));
                    }
                }

                let s_len = self.buf[0];
                let s = str::from_utf8(&self.buf[1..1 + s_len as usize])
                    .map_err(FillError::ToUtf8Failed)?;

                match record_field_content {
                    RecordFieldContent::COUNTRY(i, v, v_name) => {
                        match n_loop {
                            0 => {
                                *v = s.into();
                                self.static_cache.insert(*i, s.into());

                                n_loop += 1;
                                // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L252
                                // Not 1 + len
                                self.buf.rotate_left(COUNTRY_NAME_INDEX_OFFSET);
                                n_read -= COUNTRY_NAME_INDEX_OFFSET;

                                continue;
                            }
                            1 => {
                                *v_name = s.into();
                                self.static_cache
                                    .insert(*i + COUNTRY_NAME_INDEX_OFFSET as u32, s.into());
                            }
                            _ => unreachable!(),
                        }
                    }
                    #[allow(unused_variables)]
                    RecordFieldContent::REGION(i, v) => {
                        *v = s.into();
                        #[cfg(feature = "lru")]
                        {
                            self.lru_cache.push(*i, s.into());
                        }
                    }
                    #[allow(unused_variables)]
                    RecordFieldContent::CITY(i, v) => {
                        *v = s.into();
                        #[cfg(feature = "lru")]
                        {
                            self.lru_cache.push(*i, s.into());
                        }
                    }
                    RecordFieldContent::ISP(_, v) => {
                        *v = s.into();
                    }
                    RecordFieldContent::DOMAIN(_, v) => {
                        *v = s.into();
                    }
                    //
                    RecordFieldContent::LATITUDE(_) => {}
                    RecordFieldContent::LONGITUDE(_) => {}
                    #[allow(unused_variables)]
                    RecordFieldContent::ZIPCODE(i, v) => {
                        *v = s.into();
                        #[cfg(feature = "lru")]
                        {
                            self.lru_cache.push(*i, s.into());
                        }
                    }
                    #[allow(unused_variables)]
                    RecordFieldContent::TIMEZONE(i, v) => {
                        *v = s.into();
                        #[cfg(feature = "lru")]
                        {
                            self.lru_cache.push(*i, s.into());
                        }
                    }
                    RecordFieldContent::NETSPEED(_, v) => {
                        *v = s.into();
                    }
                    //
                    RecordFieldContent::PROXYTYPE(i, v) => {
                        *v = s.into();
                        self.static_cache.insert(*i, s.into());
                    }
                    RecordFieldContent::USAGETYPE(i, v) => {
                        *v = s.into();
                        self.static_cache.insert(*i, s.into());
                    }
                    RecordFieldContent::ASN(_, v) => {
                        *v = s.into();
                    }
                    RecordFieldContent::AS(_, v) => {
                        *v = s.into();
                    }
                    RecordFieldContent::LASTSEEN(_, v) => {
                        *v = s.into();
                    }
                    RecordFieldContent::THREAT(_, v) => {
                        *v = s.into();
                    }
                    RecordFieldContent::RESIDENTIAL(_, v) => {
                        *v = s.into();
                    }
                    RecordFieldContent::PROVIDER(_, v) => {
                        *v = s.into();
                    }
                }

                break;
            }
        }

        Ok(())
    }
}

//
#[derive(Debug)]
pub enum FillError {
    SeekFailed(IoError),
    ReadFailed(IoError),
    ToUtf8Failed(str::Utf8Error),
    Other(&'static str),
}

impl fmt::Display for FillError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FillError {}
