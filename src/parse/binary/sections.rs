use super::{Parse, ParseError};
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
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (n, len) = Byte::parse(&data[length..])?;
        length += len;
        let (size, len) = U32::parse(&data[length..])?;
        let size = *size as usize;
        length += len;
        let data = &data[length..(length + size as usize)];
        length += size;

        Ok((
            match *n {
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
                n => return Err(ParseError::new(format!("Unknown section number {}", n))),
            },
            length,
        ))
    }
}
