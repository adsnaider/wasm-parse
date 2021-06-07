//! Web assembly data definition.

use super::indices::MemIdx;
use super::instr::Expr;
use super::values::Byte;

#[derive(Debug, Clone)]
pub struct Data {
    pub init: Vec<Byte>,
    pub mode: DataMode,
}

#[derive(Debug, Clone)]
pub enum DataMode {
    Passive,
    Active { memory: MemIdx, offset: Expr },
}
