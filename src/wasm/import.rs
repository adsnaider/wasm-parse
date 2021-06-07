//! Web assembly import definition.

use super::{
    indices::TypeIdx,
    types::{GlobalType, MemType, TableType},
    values::Name,
};

#[derive(Debug, Clone)]
pub struct Import {
    pub module: Name,
    pub name: Name,
    pub desc: ImportDesc,
}

#[derive(Debug, Clone)]
pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
