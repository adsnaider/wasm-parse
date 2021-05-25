use super::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::values::{Byte, U32};

mod code;
mod custom;
mod data;
mod elem;
mod export;
mod func;
mod global;
mod import;
mod mem;
mod start;
mod table;
mod types;

#[derive(Debug)]
pub enum Section {
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

impl Parse for Section {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let n = Byte::parse(data)?;
        let size = *U32::parse(data)? as usize;
        let _bytes = data.consume(size);

        Ok(match *n {
            0 => Section::Custom,
            1 => Section::Type,
            2 => Section::Import,
            3 => Section::Function,
            4 => Section::Table,
            5 => Section::Memory,
            6 => Section::Global,
            7 => Section::Export,
            8 => Section::Start,
            9 => Section::Element,
            10 => Section::Code,
            11 => Section::Data,
            12 => Section::DataCount,
            n => {
                return Err(ParseError::new(
                    data,
                    format!("Unknown section number {}", n),
                ))
            }
        })
    }
}
