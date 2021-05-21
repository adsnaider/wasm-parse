use super::{Parse, ParseError};

use crate::wasm::expr::Expr;
use crate::wasm::global::Global;
use crate::wasm::types::GlobalType;

impl Parse for Global {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (tpe, len) = GlobalType::parse(data)?;
        length += len;
        let (init, len) = Expr::parse(&data[length..])?;
        length += len;
        Ok((Global { tpe, init }, length))
    }
}
