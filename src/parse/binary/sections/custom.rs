use crate::parse::binary::{Parse, ParseError};
use crate::wasm::values::Name;

#[derive(Debug)]
pub struct CustomSection {
    pub name: Name,
    pub data: Vec<u8>,
}

impl Parse for CustomSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (name, len) = Name::parse(data)?;
        length += len;
        Ok((
            CustomSection {
                name: name,
                data: data[length..].to_owned(),
            },
            data.len(),
        ))
    }
}
