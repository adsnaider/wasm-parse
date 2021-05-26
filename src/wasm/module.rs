//! Web Assembly Module definition.

use super::{data, elem, export, func, global, import, mem, start, table, types};

#[derive(Debug)]
pub struct Module {
    pub types: Vec<types::FuncType>,
    pub funcs: Vec<func::Func>,
    pub tables: Vec<table::Table>,
    pub mems: Vec<mem::Mem>,
    pub globals: Vec<global::Global>,
    pub elems: Vec<elem::Elem>,
    pub datas: Vec<data::Data>,
    pub start: Option<start::Start>,
    pub imports: Vec<import::Import>,
    pub exports: Vec<export::Export>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            types: Vec::new(),
            funcs: Vec::new(),
            tables: Vec::new(),
            mems: Vec::new(),
            globals: Vec::new(),
            elems: Vec::new(),
            datas: Vec::new(),
            start: None,
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }
}
