use super::Parse;
use super::ParseError;
use nano_leb128::SLEB128;
use nano_leb128::ULEB128;
use std::convert::TryInto;
use std::mem::transmute;

use crate::wasm::values::{Byte, Name, F32, F64, I16, I32, I64, I8, S32, S64, U32, U64};

fn to_unsinged_8(n: i8) -> u8 {
    unsafe { transmute(n) }
}

fn to_unsinged_16(n: i16) -> u16 {
    unsafe { transmute(n) }
}
fn to_unsinged_32(n: i32) -> u32 {
    unsafe { transmute(n) }
}

fn to_unsinged_64(n: i64) -> u64 {
    unsafe { transmute(n) }
}

impl Parse for Byte {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let data = &data[..1];
        if data.len() != 1 {
            return Err(ParseError::new(
                "Can't decode byte. Buffer too small".to_string(),
            ));
        }
        Ok((Byte(data[0]), 1))
    }
}

impl Parse for U32 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (value, len) =
            ULEB128::read_from(data).or(Err(ParseError::new("Can't decode u32".to_string())))?;
        let value = u64::from(value).try_into().or(Err(ParseError::new(
            "Conversion failure. Expected u32, got u64".to_string(),
        )))?;
        Ok((U32(value), len))
    }
}

impl Parse for U64 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (value, len) =
            ULEB128::read_from(data).or(Err(ParseError::new("Can't decode u64".to_string())))?;
        Ok((U64(value.into()), len))
    }
}

impl Parse for S32 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (value, len) =
            SLEB128::read_from(data).or(Err(ParseError::new("Can't decode s32".to_string())))?;
        let value = i64::from(value).try_into().or(Err(ParseError::new(
            "Conversion failure. Expected s32, got s64".to_string(),
        )))?;
        Ok((S32(value), len))
    }
}

impl Parse for S64 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (value, len) =
            SLEB128::read_from(data).or(Err(ParseError::new("Can't decode s32".to_string())))?;
        Ok((S64(value.into()), len))
    }
}

impl Parse for I8 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (value, len) =
            SLEB128::read_from(data).or(Err(ParseError::new("Can't decode i8".to_string())))?;
        let value: i8 = i64::from(value).try_into().or(Err(ParseError::new(
            "Conversion failure. Expected i8, got i64".to_string(),
        )))?;
        Ok((I8(to_unsinged_8(value)), len))
    }
}

impl Parse for I16 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (value, len) =
            SLEB128::read_from(data).or(Err(ParseError::new("Can't decode i16".to_string())))?;
        let value: i16 = i64::from(value).try_into().or(Err(ParseError::new(
            "Conversion failure. Expected i16, got i64".to_string(),
        )))?;
        Ok((I16(to_unsinged_16(value)), len))
    }
}

impl Parse for I32 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (value, len) =
            SLEB128::read_from(data).or(Err(ParseError::new("Can't decode i32".to_string())))?;
        let value: i32 = i64::from(value).try_into().or(Err(ParseError::new(
            "Conversion failure. Expected i32, got i64".to_string(),
        )))?;
        Ok((I32(to_unsinged_32(value)), len))
    }
}

impl Parse for I64 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (value, len) =
            SLEB128::read_from(data).or(Err(ParseError::new("Can't decode i64".to_string())))?;
        let value: i64 = value.into();
        Ok((I64(to_unsinged_64(value)), len))
    }
}

impl Parse for F32 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 4 {
            return Err(ParseError::new(
                "Buffer too small for parsing f32.".to_string(),
            ));
        }
        Ok((
            F32(f32::from_le_bytes(
                data[0..4]
                    .try_into()
                    .expect("Buffer should have 4 elements"),
            )),
            4,
        ))
    }
}

impl Parse for F64 {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        if data.len() < 8 {
            return Err(ParseError::new(
                "Buffer too small for parsing f64.".to_string(),
            ));
        }
        Ok((
            F64(f64::from_le_bytes(
                data[0..8]
                    .try_into()
                    .expect("Buffer should have 8 elements"),
            )),
            4,
        ))
    }
}

impl Parse for Name {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (bytes, len) = Vec::<Byte>::parse(data)?;
        let bytes = unsafe { std::mem::transmute(bytes) };
        Ok((
            Name {
                name: String::from_utf8(bytes).or(Err(ParseError::new(
                    "Can't decode UTF-8 from bytes.".to_string(),
                )))?,
            },
            len,
        ))
    }
}

impl<T> Parse for Vec<T>
where
    T: Parse,
{
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (n, len) = U32::parse(&data[length..])?;
        length += len;
        let mut result = Vec::with_capacity(*n as usize);
        for _ in 0..*n {
            let (t, len) = T::parse(&data[length..])?;
            length += len;
            result.push(t);
        }
        Ok((result, length))
    }
}
