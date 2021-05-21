use crate::parse::binary::{Parse, ParseError};
use crate::wasm::global::Global;

#[derive(Debug)]
pub struct GlobalSection {
    globals: Vec<Global>,
}

impl Parse for GlobalSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (globals, len) = Vec::parse(data)?;
        Ok((GlobalSection { globals }, len))
    }
}
