use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::indices::TypeIdx;

pub struct FuncSection {
    pub funcs: Vec<TypeIdx>,
}

impl Parse for FuncSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let funcs = Vec::parse(data)?;
        Ok(FuncSection { funcs })
    }
}
