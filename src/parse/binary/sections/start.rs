use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::indices::FuncIdx;
use crate::wasm::start::Start;

#[derive(Debug)]
pub struct StartSection {
    pub start: Option<Start>,
}

impl Parse for StartSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.is_empty() {
            Ok(StartSection { start: None })
        } else {
            let start = Start::parse(data)?;
            Ok(StartSection { start: Some(start) })
        }
    }
}

impl Parse for Start {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let func = FuncIdx::parse(data)?;
        Ok(Start { func })
    }
}
