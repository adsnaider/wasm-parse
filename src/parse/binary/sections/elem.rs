use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::elem::{Elem, ElemMode};
//use crate::wasm::expr::Expr;
//use crate::wasm::indices::FuncIdx;
//use crate::wasm::indices::TableIdx;
use crate::wasm::types::RefType;
//use crate::wasm::values::U32;

#[derive(Debug)]
pub struct ElemSection {
    seg: Vec<Elem>,
}

impl Parse for ElemSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let seg = Vec::parse(data)?;
        Ok(ElemSection { seg })
    }
}

impl Parse for Elem {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't parse element. Buffer too small.".to_string(),
            ));
        }
        // TODO: Do this
        /*
        let elem = match data.consume(()) {
            0x00 => {
                let expr = Expr::parse(data)?;
                let y: Vec<FuncIdx> = Vec::parse(data)?;
                // TODO: Fix expr.
                Elem {
                    tpe: RefType::FuncRef,
                    init: Vec::new(),
                    mode: ElemMode::Active {
                        table: TableIdx(U32(0)),
                        offset: expr,
                    },
                }
            }
        };
        */
        let elem = Elem {
            tpe: RefType::FuncRef,
            init: Vec::new(),
            mode: ElemMode::Passive,
        };
        Ok(elem)
    }
}
