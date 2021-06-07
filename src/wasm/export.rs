//! Web assembly export definition.

use super::{
    indices::{FuncIdx, GlobalIdx, MemIdx, TableIdx},
    values::Name,
};

#[derive(Debug, Clone)]
pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}

#[derive(Debug, Clone)]
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}
