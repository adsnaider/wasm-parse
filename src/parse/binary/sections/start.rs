use crate::parse::binary::{Parse, ParseError};
use crate::wasm::indices::FuncIdx;
use crate::wasm::start::Start;

#[derive(Debug)]
pub struct StartSection {
    start: Start,
}

impl Parse for StartSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (start, len) = Start::parse(data)?;
        Ok((StartSection { start }, len))
    }
}

impl Parse for Start {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (func, len) = FuncIdx::parse(data)?;
        Ok((Start { func }, len))
    }
}
