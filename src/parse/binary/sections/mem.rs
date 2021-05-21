use crate::parse::binary::{Parse, ParseError};
use crate::wasm::types::MemType;

#[derive(Debug)]
pub struct MemSection {
    mems: Vec<MemType>,
}

impl Parse for MemSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (mems, len) = Vec::parse(data)?;
        Ok((MemSection { mems }, len))
    }
}
