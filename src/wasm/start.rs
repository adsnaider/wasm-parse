//! Web assembly start function definition.

use super::indices::FuncIdx;

#[derive(Debug)]
pub struct Start {
    pub func: FuncIdx,
}
