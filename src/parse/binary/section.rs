use super::Parse;
use nano_leb128::ULEB128;
use std::convert::TryInto;
use thiserror::Error;

mod code;
mod data;
mod elem;
mod export;
mod functype;
mod global;
mod import;
mod mem;
mod start;
mod table;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The section is too small")]
    SectionTooSmall,
    #[error("Couldn't decode the data")]
    DecodeError,
    #[error("Section type invalid: Got {got:}")]
    InvalidSectionType { got: u8 },
    #[error("Module malformed!")]
    MalformedModule,
}
#[derive(Debug)]
pub enum SectionType {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    DataCount,
}

pub struct Section<'a> {
    pub tpe: SectionType,
    pub data: &'a [u8],
}

impl<'a> Section<'a> {
    pub fn new(tpe: SectionType, data: &'a [u8]) -> Section<'a> {
        Section { tpe, data }
    }
}

impl<'a> Parse<'a> for Section<'a> {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, ParseError> {
        let n: u8 = data.get(0).ok_or(ParseError::SectionTooSmall)?.clone();
        let (size, len) = ULEB128::read_from(&data.get(1..).ok_or(ParseError::SectionTooSmall)?)
            .or(Err(ParseError::DecodeError))?;
        let size: usize = u64::from(size)
            .try_into()
            .or(Err(ParseError::DecodeError))?;

        let section = data
            .get(len + 1..len + 1 + size)
            .ok_or(ParseError::DecodeError)?;
        *data = data
            .get(1 + len + size..)
            .ok_or(ParseError::MalformedModule)?;

        Ok(Section::new(
            match n {
                0 => SectionType::Custom,
                1 => SectionType::Type,
                2 => SectionType::Import,
                3 => SectionType::Function,
                4 => SectionType::Table,
                5 => SectionType::Memory,
                6 => SectionType::Global,
                7 => SectionType::Export,
                8 => SectionType::Start,
                9 => SectionType::Element,
                10 => SectionType::Code,
                11 => SectionType::Data,
                12 => SectionType::DataCount,
                _ => return Err(ParseError::InvalidSectionType { got: n }),
            },
            &section,
        ))
    }
}

impl<'a> std::fmt::Debug for Section<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point").field("type", &self.tpe).finish()
    }
}
