use thiserror::Error;

pub mod name;
pub mod preamble;
pub mod section;
pub mod vector;

pub trait Parse<'a>: Sized {
    type Error: std::error::Error + std::fmt::Display + std::fmt::Debug + 'static;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error>;
    fn parse_into(&mut self, data: &mut &'a [u8]) -> Result<(), Self::Error> {
        *self = Self::parse(data)?;
        Ok(())
    }
}

pub struct BinaryModule<'a> {
    pub header: preamble::Preamble,
    pub sections: Vec<section::Section<'a>>,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Header error")]
    HeaderError(#[from] preamble::ParseError),
    #[error("Section error")]
    SectionError(#[from] section::ParseError),
    #[error("Couldn't read buffer: buffer too small for parsing")]
    DataTooSmall,
    #[error("Other parsing error: {0:?}")]
    OtherError(#[from] Option<Box<dyn std::error::Error>>),
}

impl<'a> Parse<'a> for BinaryModule<'a> {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error> {
        let header = preamble::Preamble::parse(data)?;
        let mut sections = Vec::new();
        while !data.is_empty() {
            sections.push(section::Section::parse(data)?);
        }

        Ok(BinaryModule { header, sections })
    }
}

impl<'a> Parse<'a> for u8 {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error> {
        let byte = data.get(0).ok_or(ParseError::DataTooSmall)?;
        *data = &data[1..];
        Ok(*byte)
    }
}
