use crate::parse::binary::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::export::{Export, ExportDesc};
use crate::wasm::indices::{FuncIdx, GlobalIdx, MemIdx, TableIdx};
use crate::wasm::values::Name;

#[derive(Debug, Default)]
pub struct ExportSection {
    pub exports: Vec<Export>,
}

impl Parse for ExportSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let exports = Vec::parse(data)?;
        Ok(ExportSection { exports })
    }
}

impl Parse for Export {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let name = Name::parse(data)?;
        let desc = ExportDesc::parse(data)?;
        Ok(Export { name, desc })
    }
}

impl Parse for ExportDesc {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't read export description. Buffer too small.".to_string(),
            ));
        }
        match data.consume(()) {
            0x00 => {
                let x = FuncIdx::parse(data)?;
                Ok(ExportDesc::Func(x))
            }
            0x01 => {
                let x = TableIdx::parse(data)?;
                Ok(ExportDesc::Table(x))
            }
            0x02 => {
                let x = MemIdx::parse(data)?;
                Ok(ExportDesc::Mem(x))
            }
            0x03 => {
                let x = GlobalIdx::parse(data)?;
                Ok(ExportDesc::Global(x))
            }
            x => Err(ParseError::new(
                data,
                format!("Unknown export description type {:X}", x),
            )),
        }
    }
}
