//! Web assembly data definition.

use super::indices::MemIdx;
use super::instr::Expr;

#[derive(Debug)]
pub struct Data {
    pub init: Vec<u8>,
    pub mode: DataMode,
}

#[derive(Debug)]
pub enum DataMode {
    Passive,
    Active { memory: MemIdx, offset: Expr },
}
