//! Wasm structure/model
//!
//! This module contains all the necessary definitions to define a web assembly module. The module
//! is built through composition where larger structures contain the smaller structures.

pub mod data;
pub mod elem;
pub mod export;
pub mod func;
pub mod global;
pub mod import;
pub mod indices;
pub mod instr;
pub mod mem;
pub mod module;
pub mod start;
pub mod table;
pub mod types;
pub mod values;
