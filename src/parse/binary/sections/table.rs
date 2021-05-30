use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::table::Table;
use crate::wasm::types::TableType;

#[derive(Debug, Default)]
pub struct TableSection {
    pub tables: Vec<Table>,
}

impl Parse for TableSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tables = Vec::parse(data)?;
        Ok(TableSection { tables })
    }
}

impl Parse for Table {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = TableType::parse(data)?;
        Ok(Table { tpe })
    }
}
