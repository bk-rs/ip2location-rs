use core::{fmt, ops::ControlFlow};
use std::io::{BufRead, Error as IoError, Read as _};

use crate::bin_format::field::Field;

//
pub const LEN: usize = 5 + 6 * 4 + 2 + 4;

//
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct Header {
    pub r#type: Type,
    pub num_fields: u8,
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub ipv4_data_count: u32,
    pub ipv4_data_base_index: u32,
    pub ipv6_data_count: u32,
    pub ipv6_data_base_index: u32,
    pub ipv4_index_base_index: u32,
    pub ipv6_index_base_index: u32,
    pub product_code: u8,
    pub license_code: u8,
    pub size: u32,
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
#[derive(Debug)]
pub struct Parser {
    header: Header,
    state: ParserState,
    buf: Vec<u8>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ParserState {
    Idle,
    TypeParsed,
    NumFieldsParsed,
}

impl Default for ParserState {
    fn default() -> Self {
        Self::Idle
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            header: Default::default(),
            state: Default::default(),
            // core::mem::size_of::<u32>()
            buf: vec![0; 4],
        }
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
                    let value = self.buf[0];

                    let r#type =
                        Type::try_from(value).map_err(|_| ParseError::TypeValueInvalid(value))?;

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
                    let value = self.buf[0];

                    let num_fields = value;

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

        Ok(ControlFlow::Break((n_parsed, self.header)))
    }
}

//
#[derive(Debug)]
pub enum ParseError {
    ReadFailed(IoError),
    TypeValueInvalid(u8),
    NumFieldsMismatch(u8),
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

    #[test]
    fn test_parse() -> Result<(), Box<dyn error::Error>> {
        for (path, r#type) in &[
            ("data/IP2PROXY-LITE-PX1.BIN", Type::PX1),
            ("data/IP2PROXY-LITE-PX11.BIN", Type::PX11),
        ] {
            let mut f = match File::open(path) {
                Ok(x) => x,
                Err(err) if err.kind() == IoErrorKind::NotFound => {
                    eprintln!("path: {}, err: {:?}", path, err);
                    return Ok(());
                }
                Err(err) => return Err(err.into()),
            };
            let mut buf = vec![0; LEN];
            f.read_exact(&mut buf)?;

            println!("buf: {:?}", buf);

            //
            let mut parser = Parser::new();
            match parser.parse(&mut Cursor::new(buf))? {
                ControlFlow::Break((n_parsed, header)) => {
                    assert_eq!(n_parsed, 2);
                    assert_eq!(header.r#type, *r#type);

                    println!("parser: {:?}", parser);
                }
                x => {
                    println!("parser: {:?}", parser);
                    panic!("{:?}", x)
                }
            }
        }

        Ok(())
    }
}
