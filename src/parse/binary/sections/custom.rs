use crate::parse::binary::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::values::Name;

#[derive(Debug)]
pub struct CustomSection {
    pub name: Name,
    pub data: Vec<u8>,
}

impl Parse for CustomSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let name =
            Name::parse(data).map_err(|err| err.extend("Can't parse custom section name"))?;
        Ok(CustomSection {
            name: name,
            data: (*data.read(..)).to_owned(),
        })
    }
}
