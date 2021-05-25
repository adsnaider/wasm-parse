use super::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::values::U32;

use crate::wasm::types::{
    FuncType, GlobalType, Limits, MemType, Mutability, NumType, RefType, ResultType, TableType,
    ValType,
};

impl Parse for NumType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't read number type. Buffer to small".to_string(),
            ));
        }
        match data.consume(()) {
            0x7F => Ok(NumType::I32),
            0x7E => Ok(NumType::I64),
            0x7D => Ok(NumType::F32),
            0x7C => Ok(NumType::F64),
            x => Err(ParseError::new(
                data,
                format!("Unknown number type {:X?}", x),
            )),
        }
    }
}

impl Parse for RefType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't read reference type. Buffer to small".to_string(),
            ));
        }
        match data.consume(()) {
            0x70 => Ok(RefType::FuncRef),
            0x6F => Ok(RefType::ExternRef),
            x => Err(ParseError::new(
                data,
                format!("Unknown reference type {:X?}", x),
            )),
        }
    }
}

impl Parse for ValType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't read value type. Buffer to small".to_string(),
            ));
        }
        match data.consume(()) {
            0x7F => Ok(ValType::Num(NumType::I32)),
            0x7E => Ok(ValType::Num(NumType::I64)),
            0x7D => Ok(ValType::Num(NumType::F32)),
            0x7C => Ok(ValType::Num(NumType::F64)),
            0x70 => Ok(ValType::Ref(RefType::FuncRef)),
            0x6F => Ok(ValType::Ref(RefType::ExternRef)),
            x => Err(ParseError::new(
                data,
                format!("Unknown value type {:X?}", x),
            )),
        }
    }
}

impl Parse for ResultType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let types = Vec::parse(data).map_err(|err| err.extend("Can't parse ValType"))?;
        Ok(ResultType { types })
    }
}

impl Parse for FuncType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't read function type. Buffer to small".to_string(),
            ));
        }
        let magic = data.consume(());
        if magic != 0x60u8 {
            return Err(ParseError::new(
                data,
                format!(
                    "Incorrect function magic: Got {:X}, Expected {:X}",
                    magic, 0x60
                ),
            ));
        }
        let params = ResultType::parse(data)
            .map_err(|err| err.extend("Can't parse ResultType parameters"))?;
        let result =
            ResultType::parse(data).map_err(|err| err.extend("Can't parse ResultType result"))?;
        Ok(FuncType { params, result })
    }
}

impl Parse for Limits {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't read limit type. Buffer to small".to_string(),
            ));
        }

        let (min, max) = match data.consume(()) {
            0x00 => {
                let min = U32::parse(data).map_err(|err| err.extend("Can't get U32 limit min"))?;
                (min, None)
            }
            0x01 => {
                let min = U32::parse(data).map_err(|err| err.extend("Can't get U32 limit min"))?;
                let max = U32::parse(data).map_err(|err| err.extend("Can't get U32 limit max"))?;
                (min, Some(max))
            }
            x => return Err(ParseError::new(data, format!("Can't decode limit {:X}", x))),
        };
        Ok(Limits { min, max })
    }
}

impl Parse for MemType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let lim = Limits::parse(data).map_err(|err| err.extend("Can't parse limit"))?;
        Ok(MemType { lim })
    }
}

impl Parse for TableType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = RefType::parse(data).map_err(|err| err.extend("Can't parse RefType"))?;
        let lim = Limits::parse(data).map_err(|err| err.extend("Can't parse limit"))?;
        Ok(TableType { lim, tpe })
    }
}

impl Parse for GlobalType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = ValType::parse(data).map_err(|err| err.extend("Can't parse ValType."))?;
        let mutability =
            Mutability::parse(data).map_err(|err| err.extend("Can't parse Mutability"))?;
        Ok(GlobalType { mutability, tpe })
    }
}

impl Parse for Mutability {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't read mutability type. Buffer too small".to_string(),
            ));
        }

        match data.consume(()) {
            0x00 => Ok(Mutability::Const),
            0x01 => Ok(Mutability::Mut),
            x => Err(ParseError::new(
                data,
                format!("Invalid mutability encoding {:X}", x),
            )),
        }
    }
}
