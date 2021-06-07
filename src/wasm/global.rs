//! Web assembly Global definition.

use super::{instr::Expr, types::GlobalType};

#[derive(Debug, Clone)]
pub struct Global {
    pub tpe: GlobalType,
    pub init: Expr,
}
