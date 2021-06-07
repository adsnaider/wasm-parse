//! Web assembly table definition.

use super::types::TableType;

#[derive(Debug, Clone)]
pub struct Table {
    pub tpe: TableType,
}
