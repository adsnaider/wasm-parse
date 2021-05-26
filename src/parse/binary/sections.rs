use super::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::values::{Byte, U32};

pub mod code;
pub mod custom;
pub mod data;
pub mod elem;
pub mod export;
pub mod func;
pub mod global;
pub mod import;
pub mod mem;
pub mod start;
pub mod table;
pub mod types;

#[derive(Debug)]
pub enum Section {
    Custom(custom::CustomSection),
    Type(types::TypeSection),
    Import(import::ImportSection),
    Function(func::FuncSection),
    Table(table::TableSection),
    Memory(mem::MemSection),
    Global(global::GlobalSection),
    Export(export::ExportSection),
    Start(start::StartSection),
    Element(elem::ElemSection),
    Code(code::CodeSection),
    Data(data::DataSection),
    DataCount(U32),
}

impl Parse for Section {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let n = Byte::parse(data)?;
        let size = *U32::parse(data)? as usize;
        let mut bytes = data.consume(size);

        Ok(match *n {
            0 => Section::Custom(custom::CustomSection::parse(&mut bytes)?),
            1 => Section::Type(types::TypeSection::parse(&mut bytes)?),
            2 => Section::Import(import::ImportSection::parse(&mut bytes)?),
            3 => Section::Function(func::FuncSection::parse(&mut bytes)?),
            4 => Section::Table(table::TableSection::parse(&mut bytes)?),
            5 => Section::Memory(mem::MemSection::parse(&mut bytes)?),
            6 => Section::Global(global::GlobalSection::parse(&mut bytes)?),
            7 => Section::Export(export::ExportSection::parse(&mut bytes)?),
            8 => Section::Start(start::StartSection::parse(&mut bytes)?),
            9 => Section::Element(elem::ElemSection::parse(&mut bytes)?),
            10 => Section::Code(code::CodeSection::parse(&mut bytes)?),
            11 => Section::Data(data::DataSection::parse(&mut bytes)?),
            12 => Section::DataCount(U32::parse(&mut bytes)?),
            n => {
                return Err(ParseError::new(
                    data,
                    format!("Unknown section number {}", n),
                ))
            }
        })
    }
}
