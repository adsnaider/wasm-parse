use super::{Parse, ParseError, ParsingData};
use crate::wasm::instr::{Expr, Instr};

// TODO: This
impl Parse for Instr {
    fn parse(_data: &mut ParsingData) -> Result<Self, ParseError> {
        Ok(Instr::Undefined)
    }
}

impl Parse for Expr {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let instructions =
            Vec::parse(data).map_err(|err| err.extend("Couldn't parse instructions"))?;
        Ok(Expr { instructions })
    }
}
