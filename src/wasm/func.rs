use super::expr::Expr;
use super::types::ValType;
use crate::parse::binary::Parse;

use thiserror::Error;

#[derive(Debug)]
pub struct Func {
    typeidx: u32,
    locals: Vec<ValType>,
    body: Expr,
}
