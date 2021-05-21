use thiserror::Error;

pub mod global;
pub mod indices;
pub mod module;
pub mod preamble;
pub mod sections;
pub mod types;
pub mod values;
pub mod vector;

#[derive(Debug, Error)]
#[error("Parse error: {}", reason)]
pub struct ParseError {
    reason: String,
}

impl ParseError {
    pub fn new(reason: String) -> ParseError {
        ParseError { reason }
    }
}

pub trait Parse: Sized {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError>;
    fn parse_into(&mut self, data: &[u8]) -> Result<usize, ParseError> {
        let (value, len) = Self::parse(data)?;
        *self = value;
        Ok(len)
    }
}
