use crate::parse::binary::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::values::Name;

use crate::wasm::import::{Import, ImportDesc};
use crate::wasm::indices::TypeIdx;
use crate::wasm::types::{GlobalType, MemType, TableType};

#[derive(Debug, Default)]
pub struct ImportSection {
    pub imports: Vec<Import>,
}

impl Parse for ImportSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let imports = Vec::parse(data)?;
        Ok(ImportSection { imports })
    }
}

impl Parse for Import {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let module = Name::parse(data)?;
        let name = Name::parse(data)?;
        let desc = ImportDesc::parse(data)?;
        Ok(Import { module, name, desc })
    }
}

impl Parse for ImportDesc {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Invalid import description. Buffer too small.".to_string(),
            ));
        }
        match data.consume(()) {
            0x00 => {
                let tpe = TypeIdx::parse(data)?;
                Ok(ImportDesc::Func(tpe))
            }
            0x01 => {
                let tpe = TableType::parse(data)?;
                Ok(ImportDesc::Table(tpe))
            }
            0x02 => {
                let tpe = MemType::parse(data)?;
                Ok(ImportDesc::Mem(tpe))
            }
            0x03 => {
                let tpe = GlobalType::parse(data)?;
                Ok(ImportDesc::Global(tpe))
            }
            x => {
                return Err(ParseError::new(
                    data,
                    format!("Invalid Import description value: {:X}", x),
                ))
            }
        }
    }
}
