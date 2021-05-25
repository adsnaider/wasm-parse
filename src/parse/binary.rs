use std::ops::Deref;
use std::ops::RangeFull;
use thiserror::Error;

pub mod expr;
pub mod global;
pub mod indices;
pub mod instr;
pub mod module;
pub mod preamble;
pub mod sections;
pub mod types;
pub mod values;

#[derive(Debug, Error)]
#[error("Parsing error at byte {}. Description: {}", location, reason)]
pub struct ParseError {
    location: usize,
    reason: String,
}

impl ParseError {
    pub fn new(data: &ParsingData, reason: String) -> ParseError {
        ParseError {
            location: data.start,
            reason,
        }
    }

    pub fn extend(self, message: &str) -> ParseError {
        let mut err = self;
        err.reason = format!("{} --- {}", message, err.reason);
        err
    }
}

trait Consume<R> {
    type Output;
    fn read(&self, idx: R) -> Self::Output;
    fn consume(&mut self, idx: R) -> Self::Output;
}

#[derive(Debug)]
pub struct WasmBinary(Box<[u8]>);

#[derive(Debug, Clone)]
pub struct ParsingData<'a> {
    data: &'a WasmBinary,
    start: usize,
    end: usize,
}

impl Deref for WasmBinary {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Box<[u8]>> for WasmBinary {
    fn from(from: Box<[u8]>) -> WasmBinary {
        WasmBinary(from)
    }
}

impl From<Vec<u8>> for WasmBinary {
    fn from(from: Vec<u8>) -> WasmBinary {
        WasmBinary(from.into_boxed_slice())
    }
}

impl From<&[u8]> for WasmBinary {
    fn from(from: &[u8]) -> WasmBinary {
        WasmBinary(from.into())
    }
}

impl<'a> ParsingData<'a> {
    pub fn new(data: &'a WasmBinary) -> ParsingData<'a> {
        ParsingData {
            data,
            start: 0,
            end: data.len(),
        }
    }

    pub fn seek(&mut self, increment: usize) {
        self.start += increment
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() <= 0
    }
}

impl<'a> Deref for ParsingData<'a> {
    type Target = [u8];
    fn deref(&self) -> &'a Self::Target {
        &self.data[self.start..self.end]
    }
}

impl<'a> Consume<usize> for ParsingData<'a> {
    type Output = ParsingData<'a>;
    fn read(&self, mut idx: usize) -> Self::Output {
        if idx > self.len() {
            idx = self.len();
        }
        let mut data = self.clone();
        data.end = self.start + idx;
        data
    }

    fn consume(&mut self, mut idx: usize) -> Self::Output {
        if idx > self.len() {
            idx = self.len();
        }
        let data = self.read(idx);
        self.seek(idx);
        data
    }
}

impl<'a> Consume<RangeFull> for ParsingData<'a> {
    type Output = ParsingData<'a>;
    fn read(&self, _idx: RangeFull) -> Self::Output {
        self.clone()
    }

    fn consume(&mut self, idx: RangeFull) -> Self::Output {
        let data = self.read(idx);
        self.start = self.end;
        data
    }
}

impl<'a> Consume<()> for ParsingData<'a> {
    type Output = u8;
    fn read(&self, _idx: ()) -> u8 {
        assert!(self.len() > 0);
        self.data[self.start]
    }

    fn consume(&mut self, idx: ()) -> u8 {
        let data = self.read(idx);
        self.start += 1;
        data
    }
}

pub trait Parse: Sized {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError>;
    fn parse_into(&mut self, data: &mut ParsingData) -> Result<(), ParseError> {
        *self = Self::parse(data)?;
        Ok(())
    }
}
