use super::{Parse, ParseError, ParsingData};
use crate::wasm::indices::{
    DataIdx, ElemIdx, FuncIdx, GlobalIdx, LabelIdx, LocalIdx, MemIdx, TableIdx, TypeIdx,
};
use crate::wasm::values::U32;

impl Parse for FuncIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}

impl Parse for DataIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}

impl Parse for ElemIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}

impl Parse for GlobalIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}

impl Parse for LabelIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}

impl Parse for LocalIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}

impl Parse for MemIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}

impl Parse for TypeIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}

impl Parse for TableIdx {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let idx = U32::parse(data).map_err(|err| err.extend("Can't parse u32"))?;
        Ok(Self(idx))
    }
}
