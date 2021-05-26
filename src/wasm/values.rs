//! Web assembly values definitions.

use std::ops::Deref;

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Byte(pub u8);

impl Deref for Byte {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct U32(pub u32);

impl Deref for U32 {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct U64(pub u64);

impl Deref for U64 {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct S32(pub i32);

impl Deref for S32 {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct S64(pub i64);

impl Deref for S64 {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct I8(pub u8);

impl Deref for I8 {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct I16(pub u16);

impl Deref for I16 {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct I32(pub u32);

impl Deref for I32 {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct I64(pub u64);

impl Deref for I64 {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct F64(pub f64);

impl Deref for F64 {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct F32(pub f32);

impl Deref for F32 {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Name {
    pub name: String,
}
