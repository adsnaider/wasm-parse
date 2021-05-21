use crate::parse::binary::{Parse, ParseError};
use crate::wasm::types::FuncType;

#[derive(Debug)]
pub struct TypeSection {
    pub types: Vec<FuncType>,
}

impl Parse for TypeSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (types, len) = Vec::parse(data)?;
        Ok((TypeSection { types }, len))
    }
}
