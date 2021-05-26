//! Web assembly table definition.

use super::types::TableType;

#[derive(Debug)]
pub struct Table {
    pub tpe: TableType,
}
