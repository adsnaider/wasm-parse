use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::types::FuncType;

#[derive(Debug)]
pub struct TypeSection {
    pub types: Vec<FuncType>,
}

impl Parse for TypeSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let types = Vec::parse(data)?;
        Ok(TypeSection { types })
    }
}
