use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::types::MemType;

#[derive(Debug)]
pub struct MemSection {
    mems: Vec<MemType>,
}

impl Parse for MemSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let mems = Vec::parse(data)?;
        Ok(MemSection { mems })
    }
}
