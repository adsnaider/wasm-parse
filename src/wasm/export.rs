use super::{
    indices::{FuncIdx, GlobalIdx, MemIdx, TableIdx},
    values::Name,
};

#[derive(Debug)]
pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}

#[derive(Debug)]
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}
