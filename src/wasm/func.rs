use super::expr::Expr;
use super::indices::TypeIdx;
use super::types::ValType;

#[derive(Debug)]
pub struct Func {
    pub index: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}
