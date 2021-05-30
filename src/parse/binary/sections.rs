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
            0 => Section::Custom(
                custom::CustomSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse custom section."))?,
            ),
            1 => Section::Type(
                types::TypeSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse type section."))?,
            ),
            2 => Section::Import(
                import::ImportSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse import section."))?,
            ),
            3 => Section::Function(
                func::FuncSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse function section."))?,
            ),
            4 => Section::Table(
                table::TableSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse table section."))?,
            ),
            5 => Section::Memory(
                mem::MemSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse memory section."))?,
            ),
            6 => Section::Global(
                global::GlobalSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse global section."))?,
            ),
            7 => Section::Export(
                export::ExportSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse export section."))?,
            ),
            8 => Section::Start(
                start::StartSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse start section."))?,
            ),
            9 => Section::Element(
                elem::ElemSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse element section."))?,
            ),
            10 => Section::Code(
                code::CodeSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse code section."))?,
            ),
            11 => Section::Data(
                data::DataSection::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse data section."))?,
            ),
            12 => Section::DataCount(
                U32::parse(&mut bytes)
                    .map_err(|err| err.extend("Can't parse data count section."))?,
            ),
            n => {
                return Err(ParseError::new(
                    data,
                    format!("Unknown section number {}", n),
                ))
            }
        })
    }
}
