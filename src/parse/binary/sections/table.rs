use crate::parse::binary::{Parse, ParseError, ParsingData};
use crate::wasm::types::TableType;

#[derive(Debug)]
pub struct TableSection {
    tables: Vec<TableType>,
}

impl Parse for TableSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tables = Vec::parse(data)?;
        Ok(TableSection { tables })
    }
}
