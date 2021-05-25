use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::indices::FuncIdx;
use crate::wasm::start::Start;

#[derive(Debug)]
pub struct StartSection {
    start: Start,
}

impl Parse for StartSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let start = Start::parse(data)?;
        Ok(StartSection { start })
    }
}

impl Parse for Start {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let func = FuncIdx::parse(data)?;
        Ok(Start { func })
    }
}
