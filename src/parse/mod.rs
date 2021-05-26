//! Parsing elements for the web assembly.
//!
//! As of this moment, the only supported format is the binary format specified in the web assembly
//! specification. For this reason, this module will just reexport all of the componenets within
//! the binary submodule.

pub mod binary;
// TODO: Maybe change this if added support for text format.
pub use binary::*;
