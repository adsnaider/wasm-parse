//! Web assembly start function definition.

use super::indices::FuncIdx;

#[derive(Debug, Clone)]
pub struct Start {
    pub func: FuncIdx,
}
