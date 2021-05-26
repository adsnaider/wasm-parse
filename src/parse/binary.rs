//! Binary format parser
//!
//! This module defines the necessary components to create a parseable binary format object.
//! Objects that can be parsed will implement [Parse].

use std::ops::Deref;
use std::ops::RangeFull;
use thiserror::Error;

mod global;
mod indices;
mod instr;
mod module;
mod preamble;
mod sections;
mod types;
mod values;

/// Parses a chunk of data into its individual web assembly component.
///
/// # Examples
///
/// ## Usage
///
/// ```
/// # use wasm_parse::parse::binary::*;
/// # use wasm_parse::wasm::types::NumType;
/// let wasm: WasmBinary = [0x7F].as_ref().into();
/// let mut wasm = ParsingData::new(&wasm);
/// assert_eq!(NumType::parse(&mut wasm).unwrap(), NumType::I32);
/// // We consumed the input.
/// assert!(wasm.is_empty());
/// ```
///
///
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
///
/// ## Implementation
///
/// ```
/// use wasm_parse::parse::{Consume, Parse, ParseError, ParsingData};
/// struct Foo {
///     byte1: u8,
///     byte2: u8,
/// }
///
/// impl Parse for Foo {
///     fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
///         if data.len() < 2 {
///             return Err(ParseError::new(
///                 data,
///                 "We need 2 bytes to prase foo".to_string(),
///             ));
///         }
///         Ok(Foo {
///             byte1: data.consume(()),
///             byte2: data.consume(()),
///         })
///     }
/// }
/// ```

pub trait Parse: Sized {
    /// Parses a web assembly componenet.
    ///
    /// The parser consumes the data that was passed in. This simplifies the process of having to
    /// keep track of the length and/or moving pointers around.
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError>;
    /// Parses the web assembly componenet directly into the object.
    fn parse_into(&mut self, data: &mut ParsingData) -> Result<(), ParseError> {
        *self = Self::parse(data)?;
        Ok(())
    }
}
/// Error case struct for [Parse]
///
/// The [ParseError] keeps track of the location in the data stream the error occured as well as
/// the reason the error occurred.
#[derive(Debug, Error)]
#[error("Parsing error at byte {}. Description: {}", location, reason)]
pub struct ParseError {
    location: usize,
    reason: String,
}

impl ParseError {
    /// Constructs a ParseError.
    pub fn new(data: &ParsingData, reason: String) -> ParseError {
        ParseError {
            location: data.start,
            reason,
        }
    }

    /// Extends the error message by prepending the given `message` to the error's message.
    pub fn extend(self, message: &str) -> ParseError {
        let mut err = self;
        err.reason = format!("{} --- {}", message, err.reason);
        err
    }
}

/// Read or consume an input.
///
/// Similar to [`std::ops::Index`](::std::ops::Index) but gives two methods:
/// [`read()`](Self::read()) and [`consume()`](Self::consume()).
/// A call to `read()` will return the elements at the given indices while a call to `consume()`
/// reads the data and updates the internal pointer to effectively strip those elements.
pub trait Consume<R> {
    type Output;
    fn read(&self, idx: R) -> Self::Output;
    fn consume(&mut self, idx: R) -> Self::Output;
}

/// Simple wrapper for an owned [u8] slice.
#[derive(Debug)]
pub struct WasmBinary(Box<[u8]>);

/// Loose reference to the [WasmBinary].
///
/// This structure is used within the parse method. It loosely acts like an iterator in the sense
/// that it contains a reference to the [WasmBinary] and can consume data in a linear fashion. The
/// struct can be cloned which allows one to have multiple references to the binary all in different
/// offsets.
#[derive(Debug, Clone)]
pub struct ParsingData<'a> {
    data: &'a WasmBinary,
    start: usize,
    end: usize,
}

/// Dereferences into the underlying byte slice.
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

impl ParsingData<'_> {
    pub fn new(data: &WasmBinary) -> ParsingData {
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

/// Dereferences into the underlying section of the slice.
impl Deref for ParsingData<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data[self.start..self.end]
    }
}

/// Allows reading and consuming the [ParsingData].
///
/// The output of reading or consuming will be a a contiguous slice starting at the beginning of
/// the dataset and extending for a length equal to the `idx` provided. If the `idx` was larger
/// that the original length, then the slice will be truncated to fit within the original length.
///
/// All of these operations are cheap to perform as they only modify the slices without touching
/// the underlying data. The result is an object of type ParsingData witht the new range.
impl Consume<usize> for ParsingData<'_> {
    type Output = Self;
    fn read(&self, idx: usize) -> Self::Output {
        let idx = std::cmp::min(idx, self.len());
        let mut data = self.clone();
        data.end = self.start + idx;
        data
    }

    fn consume(&mut self, idx: usize) -> Self::Output {
        let idx = std::cmp::min(idx, self.len());
        let data = self.read(idx);
        self.seek(idx);
        data
    }
}

/// Reads or consumes the entire dataset.
impl Consume<RangeFull> for ParsingData<'_> {
    type Output = Self;
    fn read(&self, _idx: RangeFull) -> Self::Output {
        self.clone()
    }

    fn consume(&mut self, idx: RangeFull) -> Self::Output {
        let data = self.read(idx);
        self.start = self.end;
        data
    }
}

/// Reads or consumes one element.
impl Consume<()> for ParsingData<'_> {
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
