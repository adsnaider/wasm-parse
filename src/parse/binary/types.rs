use super::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::values::U32;

use crate::wasm::types::{
    FuncType, GlobalType, Limits, MemType, Mutability, NumType, RefType, ResultType, TableType,
    ValType,
};

/// ```
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::NumType;
/// let wasm: WasmBinary = [0x7F].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_eq!(NumType::parse(&mut wasm).unwrap(), NumType::I32);
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
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

/// ```
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::RefType;
/// let wasm: WasmBinary = [0x6F].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_eq!(RefType::parse(&mut wasm).unwrap(), RefType::ExternRef);
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
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

/// ```
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::ValType;
/// # use wasm_parse::wasm::types::RefType;
/// let wasm: WasmBinary = [0x6F].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_eq!(
///     ValType::parse(&mut wasm).unwrap(),
///     ValType::Ref(RefType::ExternRef)
/// );
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
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

/// ```
/// # #![feature(assert_matches)]
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::ResultType;
/// // 2 elements: ExternRef, F32
/// let wasm: WasmBinary = [0x02, 0x6F, 0x7D].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_matches!(ResultType::parse(&mut wasm), Ok(ResultType { types }) if types.len() == 2);
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
impl Parse for ResultType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let types = Vec::parse(data).map_err(|err| err.extend("Can't parse ValType"))?;
        Ok(ResultType { types })
    }
}

/// ```
/// # #![feature(assert_matches)]
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::FuncType;
/// // FuncType: ExternRef -> F32
/// let wasm: WasmBinary = [0x60, 0x01, 0x6F, 0x01, 0x7D].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_matches!(FuncType::parse(&mut wasm), Ok(FuncType));
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
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

/// ```
/// # #![feature(assert_matches)]
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::Limits;
/// # use wasm_parse::wasm::values::U32;
/// let wasm: WasmBinary = [0x01, 0x04, 0x05].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_matches!(
///     Limits::parse(&mut wasm),
///     Ok(Limits {
///         min: U32(4),
///         max: Some(U32(5))
///     })
/// );
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
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

/// ```
/// # #![feature(assert_matches)]
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::{Limits, MemType};
/// # use wasm_parse::wasm::values::U32;
/// let wasm: WasmBinary = [0x01, 0x04, 0x05].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_matches!(
///     MemType::parse(&mut wasm),
///     Ok(MemType {
///         lim: Limits {
///             min: U32(4),
///             max: Some(U32(5))
///         }
///     })
/// );
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
impl Parse for MemType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let lim = Limits::parse(data).map_err(|err| err.extend("Can't parse limit"))?;
        Ok(MemType { lim })
    }
}

/// ```
/// # #![feature(assert_matches)]
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::{Limits, RefType, TableType};
/// # use wasm_parse::wasm::values::U32;
/// let wasm: WasmBinary = [0x70, 0x01, 0x04, 0x05].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_matches!(
///     TableType::parse(&mut wasm),
///     Ok(TableType {
///         lim: Limits {
///             min: U32(4),
///             max: Some(U32(5))
///         },
///         tpe: RefType::FuncRef
///     })
/// );
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
impl Parse for TableType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = RefType::parse(data).map_err(|err| err.extend("Can't parse RefType"))?;
        let lim = Limits::parse(data).map_err(|err| err.extend("Can't parse limit"))?;
        Ok(TableType { lim, tpe })
    }
}

/// ```
/// # #![feature(assert_matches)]
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::{GlobalType, Mutability, ValType, NumType};
/// let wasm: WasmBinary = [0x7F, 0x00].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_matches!(
///     GlobalType::parse(&mut wasm),
///     Ok(GlobalType {
///         tpe: ValType::Num(NumType::I32),
///         mutability: Mutability::Const
///     })
/// );
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
impl Parse for GlobalType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = ValType::parse(data).map_err(|err| err.extend("Can't parse ValType."))?;
        let mutability =
            Mutability::parse(data).map_err(|err| err.extend("Can't parse Mutability"))?;
        Ok(GlobalType { mutability, tpe })
    }
}
/// ```
/// # #![feature(assert_matches)]
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::Mutability;
/// let wasm: WasmBinary = [0x00].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_matches!(Mutability::parse(&mut wasm), Ok(Mutability::Const));
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
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
