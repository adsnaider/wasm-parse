use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::global::Global;

#[derive(Debug)]
pub struct GlobalSection {
    pub globals: Vec<Global>,
}

impl Parse for GlobalSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let globals = Vec::parse(data)?;
        Ok(GlobalSection { globals })
    }
}
