use crate::wasm::instr::Expr;
use crate::wasm::types::ValType;
use crate::wasm::values::U32;

use crate::parse::binary::{Consume, Parse, ParseError, ParsingData};

#[derive(Debug, Default)]
pub struct CodeSection {
    pub code: Vec<Func>,
}

#[derive(Debug)]
pub struct Func {
    pub locals: Vec<ValType>,
    pub code: Expr,
}

struct Locals {
    pub locals: Vec<ValType>,
}

impl Parse for CodeSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        Ok(CodeSection {
            code: Vec::parse(data)?,
        })
    }
}

impl Parse for Func {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let size = *U32::parse(data)? as usize;
        let mut data = data.consume(size);
        let l: Vec<Locals> = Vec::parse(&mut data)?;
        let mut locals = Vec::new();
        for local in l {
            for tpe in local.locals {
                locals.push(tpe.clone());
            }
        }
        let code = Expr::parse(&mut data)?;
        Ok(Func { locals, code })
    }
}

impl Parse for Locals {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let n = *U32::parse(data)? as usize;
        let t = ValType::parse(data)?;
        let mut locals = Vec::with_capacity(n);
        for _ in 0..n {
            locals.push(t.clone());
        }
        Ok(Locals { locals })
    }
}
