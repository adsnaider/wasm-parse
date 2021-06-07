//! Web Assembly Module definition.

use super::{data, elem, export, func, global, import, mem, start, table, types};
use crate::parse::binary::{Parse, ParsingData, WasmBinary};
use crate::wasm::values::Name;

#[derive(Debug, Clone, Default)]
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
    pub name: Option<Name>,
}

impl Module {
    pub fn from_binary(bin: Vec<u8>) -> Module {
        let bin = WasmBinary::from(bin.into_boxed_slice());
        let mut parse = ParsingData::new(&bin);
        Module::parse(&mut parse).unwrap()
    }
}
