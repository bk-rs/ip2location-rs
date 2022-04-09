use core::{fmt, ops::ControlFlow};
use std::io::{BufRead, Error as IoError, Read as _};

use chrono::{Datelike, NaiveDate};

use crate::bin_format::{
    field::Field, ipv4_data::Ipv4DataInfo, ipv4_index::Ipv4IndexInfo, ipv6_data::Ipv6DataInfo,
    ipv6_index::Ipv6IndexInfo,
};

//
pub const LEN: usize = 5 + 6 * 4 + 2 + 4;
pub const MAX_LEN: usize = 64;

//
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct Header {
    // 1-11
    pub r#type: Type,
    // 2-13
    pub num_fields: u8,
    pub date: NaiveDate,
    pub ipv4_data_info: Ipv4DataInfo,
    pub ipv6_data_info: Ipv6DataInfo,
    pub ipv4_index_info: Ipv4IndexInfo,
    pub ipv6_index_info: Ipv6IndexInfo,
    pub product_code: u8,
    pub license_code: u8,
    pub total_size: u32,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            r#type: Default::default(),
            num_fields: Default::default(),
            date: NaiveDate::from_ymd(2000, 1, 1),
            ipv4_data_info: Default::default(),
            ipv6_data_info: Default::default(),
            ipv4_index_info: Default::default(),
            ipv6_index_info: Default::default(),
            product_code: Default::default(),
            license_code: Default::default(),
            total_size: Default::default(),
        }
    }
}

//
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    PX1,
    PX2,
    PX3,
    PX4,
    PX5,
    PX6,
    PX7,
    PX8,
    PX9,
    PX10,
    PX11,
}

impl Default for Type {
    fn default() -> Self {
        Self::PX1
    }
}

impl TryFrom<u8> for Type {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Type::PX1),
            2 => Ok(Type::PX2),
            3 => Ok(Type::PX3),
            4 => Ok(Type::PX4),
            5 => Ok(Type::PX5),
            6 => Ok(Type::PX6),
            7 => Ok(Type::PX7),
            8 => Ok(Type::PX8),
            9 => Ok(Type::PX9),
            10 => Ok(Type::PX10),
            11 => Ok(Type::PX11),
            _ => Err(()),
        }
    }
}

impl Type {
    pub fn fields(&self) -> Vec<Field> {
        Field::fields_by_type(self)
    }
}

//
//
//
#[derive(Debug, Default)]
pub struct Parser {
    header: Header,
    state: ParserState,
    buf: [u8; 4],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ParserState {
    Idle,
    TypeParsed,
    NumFieldsParsed,
    DateParsed,
    Ipv4DataInfoCountParsed,
    Ipv4DataInfoIndexStartParsed,
    Ipv6DataInfoCountParsed,
    Ipv6DataInfoIndexStartParsed,
    Ipv4IndexInfoIndexStartParsed,
    Ipv6IndexInfoIndexStartParsed,
    ProductCodeParsed,
    LicenseCodeParsed,
    TotalSizeParsed,
}

impl Default for ParserState {
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
    ) -> Result<ControlFlow<(usize, Header), usize>, ParseError> {
        let mut take = r.take(0);
        let mut n_parsed = 0_usize;

        if self.state < ParserState::TypeParsed {
            take.set_limit(1);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0 => return Ok(ControlFlow::Continue(n_parsed)),
                1 => {
                    let r#type = self.buf[0];

                    let r#type =
                        Type::try_from(r#type).map_err(|_| ParseError::TypeValueInvalid(r#type))?;

                    self.state = ParserState::TypeParsed;
                    self.header.r#type = r#type;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::NumFieldsParsed {
            take.set_limit(1);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0 => return Ok(ControlFlow::Continue(n_parsed)),
                1 => {
                    let num_fields = self.buf[0];

                    if num_fields as usize != self.header.r#type.fields().len() {
                        return Err(ParseError::NumFieldsMismatch(num_fields));
                    }

                    self.state = ParserState::NumFieldsParsed;
                    self.header.num_fields = num_fields;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::DateParsed {
            take.set_limit(3);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=2 => return Ok(ControlFlow::Continue(n_parsed)),
                3 => {
                    let year = self.buf[0];
                    let month = self.buf[1];
                    let day = self.buf[2];

                    let date = NaiveDate::from_ymd_opt(
                        (2000 + year as u16) as i32,
                        month as u32,
                        day as u32,
                    )
                    .ok_or(ParseError::YearOrMonthOrDayValueInvalid(year, month, day))?;

                    self.state = ParserState::DateParsed;
                    self.header.date = date;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::Ipv4DataInfoCountParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let count = u32::from_ne_bytes(self.buf);

                    self.state = ParserState::Ipv4DataInfoCountParsed;
                    self.header.ipv4_data_info.count = count;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::Ipv4DataInfoIndexStartParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let index_start = u32::from_ne_bytes(self.buf);

                    self.state = ParserState::Ipv4DataInfoIndexStartParsed;
                    self.header.ipv4_data_info.index_start = index_start;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::Ipv6DataInfoCountParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let count = u32::from_ne_bytes(self.buf);

                    self.state = ParserState::Ipv6DataInfoCountParsed;
                    self.header.ipv6_data_info.count = count;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::Ipv6DataInfoIndexStartParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let index_start = u32::from_ne_bytes(self.buf);

                    self.state = ParserState::Ipv6DataInfoIndexStartParsed;
                    self.header.ipv6_data_info.index_start = index_start;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::Ipv4IndexInfoIndexStartParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let index_start = u32::from_ne_bytes(self.buf);

                    self.state = ParserState::Ipv4IndexInfoIndexStartParsed;
                    self.header.ipv4_index_info.index_start = index_start;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::Ipv6IndexInfoIndexStartParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let index_start = u32::from_ne_bytes(self.buf);

                    self.state = ParserState::Ipv6IndexInfoIndexStartParsed;
                    self.header.ipv6_index_info.index_start = index_start;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::ProductCodeParsed {
            take.set_limit(1);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0 => return Ok(ControlFlow::Continue(n_parsed)),
                1 => {
                    let product_code = self.buf[0];

                    self.state = ParserState::ProductCodeParsed;
                    self.header.product_code = product_code;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::LicenseCodeParsed {
            take.set_limit(1);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0 => return Ok(ControlFlow::Continue(n_parsed)),
                1 => {
                    let license_code = self.buf[0];

                    self.state = ParserState::LicenseCodeParsed;
                    self.header.license_code = license_code;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        if self.state < ParserState::TotalSizeParsed {
            take.set_limit(4);

            let n = take.read(&mut self.buf[..])?;
            match n {
                0..=3 => return Ok(ControlFlow::Continue(n_parsed)),
                4 => {
                    let size = u32::from_ne_bytes(self.buf);

                    self.state = ParserState::TotalSizeParsed;
                    self.header.total_size = size;
                    n_parsed += n;
                }
                _ => unreachable!(),
            }
        }

        //
        // Verify
        //
        // https://github.com/ip2location/ip2proxy-rust/blob/5bdd3ef61c2e243c1b61eda1475ca23eab2b7240/src/db.rs#L173-L179
        #[allow(clippy::collapsible_if)]
        if self.header.product_code != 2 {
            if self.header.date.year() > 2020 && self.header.product_code != 0 {
                return Err(ParseError::Other("Incorrect IP2Location BIN file format. Please make sure that you are using the latest IP2Location BIN file."));
            }
        }

        if (self.header.ipv4_index_info.index_start as usize) < n_parsed {
            return Err(ParseError::Ipv4IndexInfoIndexStartTooSmall);
        }

        if self.header.ipv6_index_info.index_start < self.header.ipv4_index_info.index_end() {
            return Err(ParseError::Ipv6IndexInfoIndexStartTooSmall);
        }

        if self.header.ipv4_data_info.index_start < self.header.ipv6_index_info.index_end() {
            return Err(ParseError::Ipv4DataInfoIndexStartTooSmall);
        }

        if self.header.ipv6_data_info.index_start
            < self.header.ipv4_data_info.index_end(self.header.num_fields)
        {
            return Err(ParseError::Ipv6DataInfoIndexStartTooSmall);
        }

        if self.header.ipv6_data_info.index_end(self.header.num_fields) > self.header.total_size {
            return Err(ParseError::Ipv6DataInfoIndexStartTooLarge);
        }

        //
        self.state = ParserState::Idle;
        self.buf.fill_with(Default::default);

        Ok(ControlFlow::Break((n_parsed, self.header)))
    }
}

//
#[derive(Debug)]
pub enum ParseError {
    ReadFailed(IoError),
    TypeValueInvalid(u8),
    NumFieldsMismatch(u8),
    YearOrMonthOrDayValueInvalid(u8, u8, u8),
    Ipv4IndexInfoIndexStartTooSmall,
    Ipv6IndexInfoIndexStartTooSmall,
    Ipv4DataInfoIndexStartTooSmall,
    Ipv6DataInfoIndexStartTooSmall,
    Ipv6DataInfoIndexStartTooLarge,
    Other(&'static str),
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

    use std::{
        error,
        fs::File,
        io::{Cursor, ErrorKind as IoErrorKind},
    };

    use crate::bin_format::TEST_LITE_20220401_BIN_FILES;

    #[test]
    fn test_parse_20220401() -> Result<(), Box<dyn error::Error>> {
        for path in TEST_LITE_20220401_BIN_FILES {
            match File::open(path) {
                Ok(mut f) => {
                    let mut buf = vec![0; 64];
                    f.read_exact(&mut buf)?;

                    //
                    let mut parser = Parser::new();
                    match parser.parse(&mut Cursor::new(buf))? {
                        ControlFlow::Break((n_parsed, header)) => {
                            assert_eq!(n_parsed, LEN);
                            assert_eq!(header.date, NaiveDate::from_ymd(2022, 4, 1));

                            assert_eq!(header.ipv4_index_info.index_start, 65);
                            assert_eq!(
                                header.ipv6_index_info.index_start,
                                header.ipv4_index_info.index_end()
                            );
                            assert_eq!(
                                header.ipv4_data_info.index_start,
                                header.ipv6_index_info.index_end()
                            );
                            assert_eq!(
                                header.ipv6_data_info.index_start,
                                header.ipv4_data_info.index_end(header.num_fields)
                            );

                            println!("parser:{:?}", parser);
                        }
                        x => {
                            panic!("path:{}, ret:{:?} parser:{:?}", path, x, parser)
                        }
                    }
                }
                Err(err) if err.kind() == IoErrorKind::NotFound => {
                    eprintln!("path:{}, err:{:?}", path, err);
                }
                Err(err) => panic!("path:{}, err:{:?}", path, err),
            };
        }

        Ok(())
    }
}
