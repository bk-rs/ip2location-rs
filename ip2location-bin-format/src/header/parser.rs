use core::{fmt, ops::ControlFlow};
use std::io::{BufRead, Error as IoError, Read as _};

use super::schema::{Schema, SchemaSubType, SchemaType, VerifyError};

//
#[derive(Debug, Default)]
pub struct Parser {
    inner: Schema,
    state: State,
    buf: [u8; 4],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Idle,
    SubTypeParsed,
    NumRecordFieldsParsed,
    DateParsed,
    V4RecordsCountParsed,
    V4RecordsPositionStartParsed,
    V6RecordsCountParsed,
    V6RecordsPositionStartParsed,
    V4IndexPositionStartParsed,
    V6IndexPositionStartParsed,
    TypeParsed,
    LicenseCodeParsed,
    TotalSizeParsed,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse<R: BufRead>(
        &mut self,
        r: &mut R,
    ) -> Result<ControlFlow<(usize, Schema), usize>, ParseError> {
        let mut take = r.take(0);
        let mut n_parsed = 0_usize;

        if self.state < State::SubTypeParsed {
            take.set_limit(1);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0 => return Ok(ControlFlow::Continue(n_parsed)),
                1 => {
                    let sub_type = SchemaSubType(self.buf[0]);

                    self.state = State::SubTypeParsed;
                    self.inner.sub_type = sub_type;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::NumRecordFieldsParsed {
            take.set_limit(1);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0 => return Ok(ControlFlow::Continue(n_parsed)),
                1 => {
                    let num_record_fields = self.buf[0];

                    self.state = State::NumRecordFieldsParsed;
                    self.inner.num_record_fields = num_record_fields;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::DateParsed {
            take.set_limit(3);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=2 => return Ok(ControlFlow::Continue(n_parsed)),
                3 => {
                    let year = self.buf[0];
                    let month = self.buf[1];
                    let day = self.buf[2];

                    #[cfg(feature = "chrono")]
                    {
                        chrono::NaiveDate::from_ymd_opt(
                            (2000 + year as u16) as i32,
                            month as u32,
                            day as u32,
                        )
                        .ok_or(ParseError::YearOrMonthOrDayValueInvalid(year, month, day))?;
                    }

                    let date = (year, month, day);

                    self.state = State::DateParsed;
                    self.inner.date = date;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::V4RecordsCountParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let count = u32::from_ne_bytes(self.buf);

                    self.state = State::V4RecordsCountParsed;
                    self.inner.v4_records_count = count;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::V4RecordsPositionStartParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let position_start = u32::from_ne_bytes(self.buf);

                    self.state = State::V4RecordsPositionStartParsed;
                    self.inner.v4_records_position_start = position_start;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::V6RecordsCountParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let count = u32::from_ne_bytes(self.buf);

                    self.state = State::V6RecordsCountParsed;
                    self.inner.v6_records_count = count;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::V6RecordsPositionStartParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let position_start = u32::from_ne_bytes(self.buf);

                    self.state = State::V6RecordsPositionStartParsed;
                    self.inner.v6_records_position_start = position_start;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::V4IndexPositionStartParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let position_start = u32::from_ne_bytes(self.buf);

                    self.state = State::V4IndexPositionStartParsed;
                    self.inner.v4_index_position_start = position_start;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::V6IndexPositionStartParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let position_start = u32::from_ne_bytes(self.buf);

                    self.state = State::V6IndexPositionStartParsed;
                    self.inner.v6_index_position_start = position_start;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::TypeParsed {
            take.set_limit(1);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0 => return Ok(ControlFlow::Continue(n_parsed)),
                1 => {
                    let r#type = self.buf[0];

                    let r#type = SchemaType::try_from(r#type)
                        .map_err(|_| ParseError::TypeValueInvalid(r#type))?;

                    self.state = State::TypeParsed;
                    self.inner.r#type = r#type;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::LicenseCodeParsed {
            take.set_limit(1);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0 => return Ok(ControlFlow::Continue(n_parsed)),
                1 => {
                    let license_code = self.buf[0];

                    self.state = State::LicenseCodeParsed;
                    self.inner.license_code = license_code;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < State::TotalSizeParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let size = u32::from_ne_bytes(self.buf);

                    self.state = State::TotalSizeParsed;
                    self.inner.total_size = size;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        //
        self.inner.verify().map_err(ParseError::VerifyFailed)?;

        //
        self.state = State::Idle;
        self.buf.fill_with(Default::default);

        Ok(ControlFlow::Break((n_parsed, self.inner)))
    }
}

//
#[derive(Debug)]
pub enum ParseError {
    ReadFailed(IoError),
    YearOrMonthOrDayValueInvalid(u8, u8, u8),
    TypeValueInvalid(u8),
    VerifyFailed(VerifyError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

impl From<IoError> for ParseError {
    fn from(err: IoError) -> Self {
        Self::ReadFailed(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs::File, io::Cursor};

    use crate::{
        header::HEADER_LEN,
        test_helper::{ip2location_bin_files, ip2proxy_bin_files},
    };

    #[test]
    fn test_parse() -> Result<(), Box<dyn error::Error>> {
        for path in ip2location_bin_files().iter() {
            let mut f = File::open(&path)?;
            let mut buf = vec![0; HEADER_LEN as usize];
            f.read_exact(&mut buf)?;

            //
            let mut parser = Parser::new();
            match parser.parse(&mut Cursor::new(buf))? {
                ControlFlow::Break((_, schema)) => {
                    assert_eq!(schema.r#type, SchemaType::IP2Location);
                    println!("path:{:?}, schema:{:?}", path, schema);
                }
                x => {
                    panic!("path:{:?}, ret:{:?}, parser:{:?}", path, x, parser)
                }
            }
        }

        for path in ip2proxy_bin_files().iter() {
            let mut f = File::open(&path)?;

            let mut buf = vec![0; HEADER_LEN as usize];
            f.read_exact(&mut buf)?;

            //
            let mut parser = Parser::new();
            match parser.parse(&mut Cursor::new(buf))? {
                ControlFlow::Break((_, schema)) => {
                    assert_eq!(schema.r#type, SchemaType::IP2Proxy);
                    println!("path:{:?}, schema:{:?}", path, schema);
                }
                x => {
                    panic!("path:{:?}, ret:{:?}, parser:{:?}", path, x, parser)
                }
            }
        }

        Ok(())
    }
}
