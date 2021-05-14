use crate::parse::Parse;
use crate::wasm::types::{FuncType, NumType, RefType, ResultType, ValType};
use thiserror::Error;

#[derive(Debug)]
pub struct TypeSection {
    types: Vec<FuncType>,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Incorrect magic. Expected {expected:#X}, Got: {got:#X}")]
    InvalidMagic { expected: u8, got: u8 },
    #[error("Unknown value type: {got}")]
    UnknownValueType { got: u8 },
    #[error("Got an unknown error: {0}")]
    OtherError(#[from] Box<dyn std::error::Error>),
}

impl<'a> Parse<'a> for FuncType {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error> {
        const MAGIC: u8 = 0x60;
        if data[0] != MAGIC {
            return Err(ParseError::InvalidMagic {
                expected: MAGIC,
                got: data[0],
            });
        }
        *data = &data[1..];

        let params = ResultType::parse(data)?;
        let result = ResultType::parse(data)?;

        Ok(FuncType { params, result })
    }
}

impl<'a> Parse<'a> for ResultType {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error> {
        Ok(ResultType {
            types: Vec::<ValType>::parse(data)
                .map_err(|source| ParseError::OtherError(source.into()))?,
        })
    }
}

impl<'a> Parse<'a> for ValType {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error> {
        let byte = data[0];
        *data = &data[1..];
        match byte {
            0x7F => Ok(ValType::Num(NumType::I32)),
            0x7E => Ok(ValType::Num(NumType::I64)),
            0x7D => Ok(ValType::Num(NumType::F32)),
            0x7C => Ok(ValType::Num(NumType::F64)),
            0x70 => Ok(ValType::Ref(RefType::FuncRef)),
            0x6F => Ok(ValType::Ref(RefType::ExternRef)),
            n => Err(ParseError::UnknownValueType { got: n }),
        }
    }
}
