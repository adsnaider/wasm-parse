//! Web assembly parser.
//!
//! This can be used for verification or execution of a web assembly module.
//! The structure of this crate resembles in part the structure defined within the web assembly
//! specification with many of the names being the same.
//!
//! # Usage
// TODO: Don't ignore once it compiles.
//!
//! ```no_run
//! use wasm_parse::parse::{Parse, ParsingData, WasmBinary};
//! use wasm_parse::wasm::module::Module;
//!
//! let bytes: WasmBinary = [0x01, 0x02, 0x03, 0x04].as_ref().into();
//! let mut bytes = ParsingData::new(&bytes);
//! let module = Module::parse(&mut bytes);
//! ```

#![feature(assert_matches)]

pub mod parse;
pub mod wasm;
