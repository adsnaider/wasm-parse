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
pub enum Section<'a> {
    Custom(&'a [u8]),
    Type(&'a [functype::FuncType]),
    Import(&'a [import::Import]),
    Function(&'a [u32]),
    Table(&'a [table::Table]),
    Memory(&'a [mem::Mem]),
    Global(&'a [global::Global]),
    Export(&'a [export::Export]),
    Start(Option<&'a start::Start>),
    Element(&'a [elem::Elem]),
    Code(&'a [code::Code]),
    Data(&'a [data::Data]),
    DataCount(u32),
}

pub struct Sections<'a> {
    sections: Vec<Section<'a>>,
}

impl<'a> Parse<'a> for Section<'a> {
    type Error = ParseError;
    fn parse(data: &'a [u8]) -> Result<(Section, &'a [u8]), ParseError> {
        let n: u8 = data.get(0).ok_or(ParseError::SectionTooSmall)?.clone();
        let (size, len) = ULEB128::read_from(&data.get(1..).ok_or(ParseError::SectionTooSmall)?)
            .or(Err(ParseError::DecodeError))?;
        let size: usize = u64::from(size)
            .try_into()
            .or(Err(ParseError::DecodeError))?;

        let next = data
            .get(1 + len + size..)
            .ok_or(ParseError::MalformedModule)?;
        let section = data
            .get(len + 1..len + 1 + size)
            .ok_or(ParseError::DecodeError)?;

        Ok((
            match n {
                0 => Section::Custom(section),
                1 => Section::Type(section),
                2 => Section::Import(section),
                3 => Section::Function(section),
                4 => Section::Table(section),
                5 => Section::Memory(section),
                6 => Section::Global(section),
                7 => Section::Export(section),
                8 => Section::Start(section),
                9 => Section::Element(section),
                10 => Section::Code(section),
                11 => Section::Data(section),
                12 => Section::DataCount(section),
                _ => return Err(ParseError::InvalidSectionType { got: n }),
            },
            &next,
        ))
    }
}

impl<'a> Parse<'a> for Sections<'a> {
    type Error = ParseError;
    fn parse(data: &'a [u8]) -> Result<(Sections, &'a [u8]), ParseError> {
        let mut sections = Vec::new();
        while !data.is_empty() {
            let (section, data) = Section::parse(data)?;
            sections.push(section);
        }
        Ok((Sections { sections }, data))
    }
}

impl<'a> std::fmt::Display for Section<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Section::Custom(_) => write!(f, "{}", "Custom"),
            Section::Type(_) => write!(f, "{}", "Type"),
            Section::Import(_) => write!(f, "{}", "Import"),
            Section::Function(_) => write!(f, "{}", "Function"),
            Section::Table(_) => write!(f, "{}", "Table"),
            Section::Memory(_) => write!(f, "{}", "Memory"),
            Section::Global(_) => write!(f, "{}", "Global"),
            Section::Export(_) => write!(f, "{}", "Export"),
            Section::Start(_) => write!(f, "{}", "Start"),
            Section::Element(_) => write!(f, "{}", "Element"),
            Section::Code(_) => write!(f, "{}", "Code"),
            Section::Data(_) => write!(f, "{}", "Data"),
            Section::DataCount(_) => write!(f, "{}", "DataCount"),
        }
    }
}
