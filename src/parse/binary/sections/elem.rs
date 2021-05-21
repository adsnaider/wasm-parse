use crate::parse::binary::{Parse, ParseError};
use crate::wasm::elem::Elem;

#[derive(Debug)]
pub struct ElemSection {
    seg: Vec<Elem>,
}

impl Parse for ElemSection {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (seg, len) = Vec::parse(data)?;
        Ok((ElemSection { seg }, len))
    }
}

/*
impl Parse for Elem {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new("Can't parse element. Buffer too small."));
        }
        let (elem, len) = match data[0] {
            0x00 => {

            }
        }
    }
}
*/
