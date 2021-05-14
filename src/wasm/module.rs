use super::{data, elem, export, func, global, import, mem, start, table, types};
use std::convert::TryFrom;
use std::convert::TryInto;
use thiserror::Error;

use crate::parse::binary::section::SectionType;
use crate::parse::binary::{self, Parse};

#[derive(Debug)]
pub struct Module {
    pub types: Vec<types::FuncType>,
    pub funcs: Vec<func::Func>,
    pub tables: Vec<table::Table>,
    pub mems: Vec<mem::Mem>,
    pub globals: Vec<global::Global>,
    pub elems: Vec<elem::Elem>,
    pub datas: Vec<data::Data>,
    pub start: Option<start::Start>,
    pub imports: Vec<import::Import>,
    pub exports: Vec<export::Export>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            types: Vec::new(),
            funcs: Vec::new(),
            tables: Vec::new(),
            mems: Vec::new(),
            globals: Vec::new(),
            elems: Vec::new(),
            datas: Vec::new(),
            start: None,
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Binary format error.")]
    InvalidHeader(#[from] binary::ParseError),
    #[error("Unknown parsing error: {0:?}")]
    Other(Option<Box<dyn std::error::Error>>),
}

impl<'a> Parse<'a> for Module {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, ParseError> {
        let bin = binary::BinaryModule::parse(data)?;
        bin.try_into().into()
    }
}

impl<'a> TryFrom<binary::BinaryModule<'a>> for Module {
    type Error = ParseError;

    fn try_from(value: binary::BinaryModule<'a>) -> Result<Self, Self::Error> {
        let mut module = Module::new();
        for mut section in value.sections {
            match section.tpe {
                SectionType::Type => module
                    .types
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Import => module
                    .imports
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Function => module
                    .funcs
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Table => module
                    .tables
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Memory => module
                    .mems
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Global => module
                    .globals
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Export => module
                    .exports
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Start => module
                    .start
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Element => module
                    .elems
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Code => module
                    .funcs
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::Data => module
                    .datas
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                SectionType::DataCount => module
                    .data_count
                    .parse_into(&mut section.data)
                    .map_err(|source| ParseError::Other(Some(source.into())))?,
                _ => {}
            }
        }
        Ok(module)
    }
}
