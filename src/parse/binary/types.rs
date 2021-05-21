use super::{Parse, ParseError};
use crate::wasm::values::U32;

use crate::wasm::types::{
    ExternType, FuncType, GlobalType, Limits, MemType, Mutability, NumType, RefType, ResultType,
    TableType, ValType,
};

impl Parse for NumType {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                "Can't read number type. Buffer to small".to_string(),
            ));
        }
        match data[0] {
            0x7F => Ok((NumType::I32, 1)),
            0x7E => Ok((NumType::I64, 1)),
            0x7D => Ok((NumType::F32, 1)),
            0x7C => Ok((NumType::F64, 1)),
            x => Err(ParseError::new(format!("Unknown number type {:X?}", x))),
        }
    }
}

impl Parse for RefType {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                "Can't read reference type. Buffer to small".to_string(),
            ));
        }
        match data[0] {
            0x70 => Ok((RefType::FuncRef, 1)),
            0x6F => Ok((RefType::ExternRef, 1)),
            x => Err(ParseError::new(format!("Unknown reference type {:X?}", x))),
        }
    }
}

impl Parse for ValType {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                "Can't read value type. Buffer to small".to_string(),
            ));
        }
        match data[0] {
            0x7F => Ok((ValType::Num(NumType::I32), 1)),
            0x7E => Ok((ValType::Num(NumType::I64), 1)),
            0x7D => Ok((ValType::Num(NumType::F32), 1)),
            0x7C => Ok((ValType::Num(NumType::F64), 1)),
            0x70 => Ok((ValType::Ref(RefType::FuncRef), 1)),
            0x6F => Ok((ValType::Ref(RefType::ExternRef), 1)),
            x => Err(ParseError::new(format!("Unknown value type {:X?}", x))),
        }
    }
}

impl Parse for ResultType {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (types, len) = Vec::parse(data)?;
        Ok((ResultType { types }, len))
    }
}

impl Parse for FuncType {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                "Can't read function type. Buffer to small".to_string(),
            ));
        }
        if data[0] != 0x60 {
            return Err(ParseError::new(format!(
                "Incorrect function magic: Got {:X}, Expected {:X}",
                data[0], 0x60
            )));
        }
        let mut length = 1;
        let (params, len) = ResultType::parse(&data[length..])?;
        length += len;
        let (result, len) = ResultType::parse(&data[length..])?;
        length += len;
        Ok((FuncType { params, result }, length))
    }
}

impl Parse for Limits {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                "Can't read limit type. Buffer to small".to_string(),
            ));
        }

        let mut length = 1;
        let (min, max, len) = match data[0] {
            0x00 => {
                let (min, len) = U32::parse(&data[length..])?;
                (min, None, len)
            }
            0x01 => {
                let mut len = 0;
                let (min, l) = U32::parse(&data[length..])?;
                len += l;
                let (max, l) = U32::parse(&data[(length + len)..])?;
                len += l;
                (min, Some(max), len)
            }
            x => return Err(ParseError::new(format!("Can't decode limit {:X}", x))),
        };
        length += len;
        Ok((Limits { min, max }, length))
    }
}

impl Parse for MemType {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (lim, len) = Limits::parse(data)?;
        Ok((MemType { lim }, len))
    }
}

impl Parse for TableType {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (tpe, len) = RefType::parse(data)?;
        length += len;
        let (lim, len) = Limits::parse(&data[length..])?;
        Ok((TableType { lim, tpe }, len))
    }
}

impl Parse for GlobalType {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (tpe, len) = ValType::parse(data)?;
        length += len;
        let (mutability, len) = Mutability::parse(&data[length..])?;
        Ok((GlobalType { mutability, tpe }, len))
    }
}

impl Parse for Mutability {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                "Can't read mutability type. Buffer too small".to_string(),
            ));
        }
        Ok((
            match data[0] {
                0x00 => Mutability::Const,
                0x01 => Mutability::Mut,
                x => {
                    return Err(ParseError::new(format!(
                        "Invalid mutability encoding {:X}",
                        x
                    )));
                }
            },
            1,
        ))
    }
}
