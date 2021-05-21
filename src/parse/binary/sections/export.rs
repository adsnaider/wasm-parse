use crate::parse::binary::{Parse, ParseError};
use crate::wasm::export::{Export, ExportDesc};
use crate::wasm::indices::{FuncIdx, GlobalIdx, MemIdx, TableIdx};
use crate::wasm::values::Name;

#[derive(Debug)]
pub struct ExportSection {
    exports: Vec<Export>,
}

impl Parse for ExportSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (exports, len) = Vec::parse(data)?;
        Ok((ExportSection { exports }, len))
    }
}

impl Parse for Export {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (name, len) = Name::parse(data)?;
        length += len;
        let (desc, len) = ExportDesc::parse(&data[length..])?;
        length += len;
        Ok((Export { name, desc }, length))
    }
}

impl Parse for ExportDesc {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                "Can't read export description. Buffer too small.".to_string(),
            ));
        }
        let (desc, len) = match data[0] {
            0x00 => {
                let (x, len) = FuncIdx::parse(data)?;
                (ExportDesc::Func(x), len)
            }
            0x01 => {
                let (x, len) = TableIdx::parse(data)?;
                (ExportDesc::Table(x), len)
            }
            0x02 => {
                let (x, len) = MemIdx::parse(data)?;
                (ExportDesc::Mem(x), len)
            }
            0x03 => {
                let (x, len) = GlobalIdx::parse(data)?;
                (ExportDesc::Global(x), len)
            }
        };
        Ok((desc, len + 1))
    }
}
