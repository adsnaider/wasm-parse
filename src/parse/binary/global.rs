use super::{Parse, ParseError, ParsingData};

use crate::wasm::global::Global;
use crate::wasm::instr::Expr;
use crate::wasm::types::GlobalType;

impl Parse for Global {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = GlobalType::parse(data).map_err(|err| err.extend("Can't parse global type"))?;
        let init = Expr::parse(data).map_err(|err| err.extend("Can't parse init expression"))?;
        Ok(Global { tpe, init })
    }
}
