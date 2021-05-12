use thiserror::Error;

pub mod preamble;
pub mod section;
pub mod vector;

pub trait Parse<'a>: Sized {
    type Error: std::error::Error + std::fmt::Display + std::fmt::Debug + 'static;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error>;
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
    #[error("Uknnown parsing error: {message}")]
    UnknownError { message: String },
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
