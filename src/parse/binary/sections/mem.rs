use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::mem::Mem;
use crate::wasm::types::MemType;

#[derive(Debug)]
pub struct MemSection {
    pub mems: Vec<Mem>,
}

impl Parse for MemSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let mems = Vec::parse(data)?;
        Ok(MemSection { mems })
    }
}

impl Parse for Mem {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = MemType::parse(data)?;
        Ok(Mem { tpe })
    }
}
