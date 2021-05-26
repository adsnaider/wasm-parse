//! Web assembly memory definition.

use super::types::MemType;

#[derive(Debug)]
pub struct Mem {
    pub tpe: MemType,
}
