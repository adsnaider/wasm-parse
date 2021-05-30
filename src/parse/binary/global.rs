use super::{Parse, ParseError, ParsingData};

use crate::wasm::global::Global;
use crate::wasm::instr::Expr;
use crate::wasm::types::GlobalType;

impl Parse for Global {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = GlobalType::parse(data).map_err(|err| err.extend("Can't parse global type"))?;
        let init = Expr::parse(data).map_err(|err| err.extend("Can't parse init expression"))?;
        let global = Global { tpe, init };
        Ok(global)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global() {
        use crate::parse::binary::*;
        use crate::wasm::types::{GlobalType, Mutability, NumType, ValType};
        let wasm: WasmBinary = [0x7F, 0x01, 0x0B].as_ref().into();
        let mut wasm = ParsingData::new(&wasm);
        assert_matches!(
            Global::parse(&mut wasm),
            Ok(Global {
                tpe: GlobalType {
                    mutability: Mutability::Mut,
                    tpe: ValType::Num(NumType::I32)
                },
                init: _expr
            })
        );
        // We consumed the input.
        assert!(wasm.is_empty());
    }
}
