use crate::parse::binary::{Parse, ParseError};
use crate::wasm::types::TableType;

#[derive(Debug)]
pub struct TableSection {
    tables: Vec<TableType>,
}

impl Parse for TableSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (tables, len) = Vec::parse(data)?;
        Ok((TableSection { tables }, len))
    }
}
