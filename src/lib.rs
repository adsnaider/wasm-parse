//! Web assembly parser.
//!
//! This can be used for verification or execution of a web assembly module.
//! The structure of this crate resembles in part the structure defined within the web assembly
//! specification with many of the names being the same.
//!
//! # Usage
//!
//! ```
//! use std::fs;
//!
//! use wasm_parse::parse::{Parse, ParsingData, WasmBinary};
//! use wasm_parse::wasm::module::Module;
//!
//! let bytes: WasmBinary = fs::read("wasm-examples/hello.wasm")?.as_slice().into();
//! let mut bytes = ParsingData::new(&bytes);
//! let module = Module::parse(&mut bytes);
//! assert!(module.is_ok(), "Error: {}", module.err().unwrap());
//! assert!(bytes.is_empty());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![feature(assert_matches)]

pub mod parse;
pub mod wasm;
