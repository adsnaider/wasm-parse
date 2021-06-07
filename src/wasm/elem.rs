//! Web assembly element definition.

use super::{indices::TableIdx, instr::Expr, types::RefType};

#[derive(Debug, Clone)]
pub struct Elem {
    pub tpe: RefType,
    pub init: Vec<Expr>,
    pub mode: ElemMode,
}

#[derive(Debug, Clone)]
pub enum ElemMode {
    Passive,
    Active { table: TableIdx, offset: Expr },
    Declarative,
}
