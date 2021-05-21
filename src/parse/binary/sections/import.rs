use crate::parse::binary::{Parse, ParseError};
use crate::wasm::values::Name;

use crate::wasm::import::{Import, ImportDesc};
use crate::wasm::indices::TypeIdx;
use crate::wasm::types::{GlobalType, MemType, TableType};

#[derive(Debug)]
pub struct ImportSection {
    pub imports: Vec<Import>,
}

impl Parse for ImportSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (imports, len) = Vec::parse(data)?;
        Ok((ImportSection { imports }, len))
    }
}

impl Parse for Import {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (module, len) = Name::parse(data)?;
        length += len;
        let (name, len) = Name::parse(&data[length..])?;
        length += len;
        let (desc, len) = ImportDesc::parse(&data[length..])?;
        length += len;
        Ok((Import { module, name, desc }, length))
    }
}

impl Parse for ImportDesc {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                "Invalid import description. Buffer too small.".to_string(),
            ));
        }
        let mut length = 1;
        let (desc, len) = match data[0] {
            0x00 => {
                let (tpe, len) = TypeIdx::parse(&data[length..])?;
                (ImportDesc::Func(tpe), len)
            }
            0x01 => {
                let (tpe, len) = TableType::parse(&data[length..])?;
                (ImportDesc::Table(tpe), len)
            }
            0x02 => {
                let (tpe, len) = MemType::parse(&data[length..])?;
                (ImportDesc::Mem(tpe), len)
            }
            0x03 => {
                let (tpe, len) = GlobalType::parse(&data[length..])?;
                (ImportDesc::Global(tpe), len)
            }
            x => {
                return Err(ParseError::new(format!(
                    "Invalid Import description value: {:X}",
                    x
                )))
            }
        };
        length += len;
        Ok((desc, length))
    }
}
