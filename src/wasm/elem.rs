use super::{expr::Expr, indices::TableIdx, types::RefType};

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
