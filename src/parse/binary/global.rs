use super::{Parse, ParseError, ParsingData};

use crate::wasm::global::Global;
use crate::wasm::instr::Expr;
use crate::wasm::types::GlobalType;

/// ```
/// # #![feature(assert_matches)]
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::global::Global;
/// # use wasm_parse::wasm::types::{GlobalType, Mutability, ValType, NumType};
/// # use wasm_parse::wasm::instr::Expr;
/// let wasm: WasmBinary = [0x7F, 0x01, 0x0B].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_matches!(
///     Global::parse(&mut wasm),
///     Ok(Global {
///         tpe: GlobalType {
///             mutability: Mutability::Mut,
///             tpe: ValType::Num(NumType::I32)
///         },
///         init: Expr
///     })
/// );
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
impl Parse for Global {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = GlobalType::parse(data).map_err(|err| err.extend("Can't parse global type"))?;
        let init = Expr::parse(data).map_err(|err| err.extend("Can't parse init expression"))?;
        Ok(Global { tpe, init })
    }
}
