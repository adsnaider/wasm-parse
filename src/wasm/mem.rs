//! Web assembly memory definition.

use super::types::MemType;

#[derive(Debug, Clone)]
pub struct Mem {
    pub tpe: MemType,
}
