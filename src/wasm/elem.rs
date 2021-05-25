use super::{indices::TableIdx, instr::Expr, types::RefType};

#[derive(Debug)]
pub struct Elem {
    pub tpe: RefType,
    pub init: Vec<Expr>,
    pub mode: ElemMode,
}

#[derive(Debug)]
pub enum ElemMode {
    Passive,
    Active { table: TableIdx, offset: Expr },
    Declarative,
}
