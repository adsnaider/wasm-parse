use std::convert::TryInto;

use nano_leb128::SLEB128;
use nano_leb128::ULEB128;

use super::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::values::{Byte, Name, F32, F64, I16, I32, I64, I8, S32, S64, U32, U64};

impl Parse for Byte {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't decode byte. Buffer too small".to_string(),
            ));
        }
        Ok(Byte(data.consume(())))
    }
}

impl Parse for U32 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let (value, len) = ULEB128::read_from(&*data.read(..))
            .or(Err(ParseError::new(data, "Can't decode u32".to_string())))?;
        let value = u64::from(value).try_into().or(Err(ParseError::new(
            data,
            "Conversion failure. Expected u32, got u64".to_string(),
        )))?;
        data.consume(len);
        Ok(U32(value))
    }
}

impl Parse for U64 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let (value, len) = ULEB128::read_from(&data.read(..))
            .or(Err(ParseError::new(data, "Can't decode u64".to_string())))?;
        data.consume(len);
        Ok(U64(value.into()))
    }
}

impl Parse for S32 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let (value, len) = SLEB128::read_from(&data.read(..))
            .or(Err(ParseError::new(data, "Can't decode s32".to_string())))?;
        let value = i64::from(value).try_into().or(Err(ParseError::new(
            data,
            "Conversion failure. Expected s32, got s64".to_string(),
        )))?;
        data.consume(len);
        Ok(S32(value))
    }
}

impl Parse for S64 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let (value, len) = SLEB128::read_from(&data.read(..))
            .or(Err(ParseError::new(data, "Can't decode s32".to_string())))?;
        data.consume(len);
        Ok(S64(value.into()))
    }
}

impl Parse for I8 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let (value, len) = SLEB128::read_from(&data.read(..))
            .or(Err(ParseError::new(data, "Can't decode i8".to_string())))?;
        let value: i8 = i64::from(value).try_into().or(Err(ParseError::new(
            data,
            "Conversion failure. Expected i8, got i64".to_string(),
        )))?;
        data.consume(len);
        Ok(I8(value as u8))
    }
}

impl Parse for I16 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let (value, len) = SLEB128::read_from(&data.read(..))
            .or(Err(ParseError::new(data, "Can't decode i16".to_string())))?;
        let value: i16 = i64::from(value).try_into().or(Err(ParseError::new(
            data,
            "Conversion failure. Expected i16, got i64".to_string(),
        )))?;
        data.consume(len);
        Ok(I16(value as u16))
    }
}

impl Parse for I32 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let (value, len) = SLEB128::read_from(&data.read(..))
            .or(Err(ParseError::new(data, "Can't decode i32".to_string())))?;
        let value: i32 = i64::from(value).try_into().or(Err(ParseError::new(
            data,
            "Conversion failure. Expected i32, got i64".to_string(),
        )))?;
        data.consume(len);
        Ok(I32(value as u32))
    }
}

impl Parse for I64 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let (value, len) = SLEB128::read_from(&data.read(..))
            .or(Err(ParseError::new(data, "Can't decode i64".to_string())))?;
        let value: i64 = value.into();
        data.consume(len);
        Ok(I64(value as u64))
    }
}

impl Parse for F32 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 4 {
            return Err(ParseError::new(
                data,
                "Buffer too small for parsing f32.".to_string(),
            ));
        }
        Ok(F32(f32::from_le_bytes({
            let bytes: &[u8] = &data.consume(4);
            bytes.try_into().expect("Buffer should have 4 elements")
        })))
    }
}

impl Parse for F64 {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 8 {
            return Err(ParseError::new(
                data,
                "Buffer too small for parsing f64.".to_string(),
            ));
        }
        Ok(F64(f64::from_le_bytes({
            let bytes: &[u8] = &data.consume(8);
            bytes.try_into().expect("Buffer should have 8 elements")
        })))
    }
}

impl Parse for Name {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let bytes =
            Vec::<Byte>::parse(data).map_err(|err| err.extend("Can't parse byte vector."))?;
        // Unsafe explanation: This should be okay since Byte and u8 have the same memory
        // representation (transparent).
        let bytes = unsafe { std::mem::transmute(bytes) };
        Ok(Name {
            name: String::from_utf8(bytes).or(Err(ParseError::new(
                data,
                "Can't decode UTF-8 from bytes.".to_string(),
            )))?,
        })
    }
}

impl<T> Parse for Vec<T>
where
    T: Parse,
{
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let n = U32::parse(data).map_err(|err| err.extend("Couldn't read U32"))?;
        let mut result = Vec::with_capacity(*n as usize);
        for _ in 0..*n {
            let t = T::parse(data).map_err(|err| err.extend("Couldn't read data type."))?;
            result.push(t);
        }
        Ok(result)
    }
}
