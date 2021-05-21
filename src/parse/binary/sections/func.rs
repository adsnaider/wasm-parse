use crate::parse::binary::{Parse, ParseError};
use crate::wasm::indices::TypeIdx;

pub struct FuncSection {
    pub funcs: Vec<TypeIdx>,
}

impl Parse for FuncSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (funcs, len) = Vec::parse(data)?;
        Ok((FuncSection { funcs }, len))
    }
}
