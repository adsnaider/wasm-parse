use super::{Parse, ParseError};
use crate::wasm::indices::{
    DataIdx, ElemIdx, FuncIdx, GlobalIdx, LabelIdx, LocalIdx, MemIdx, TableIdx, TypeIdx,
};
use crate::wasm::values::U32;

impl Parse for FuncIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}

impl Parse for DataIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}

impl Parse for ElemIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}

impl Parse for GlobalIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}

impl Parse for LabelIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}

impl Parse for LocalIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}

impl Parse for MemIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}

impl Parse for TypeIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}

impl Parse for TableIdx {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let (idx, len) = U32::parse(data)?;
        Ok((Self(idx), len))
    }
}
