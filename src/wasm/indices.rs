//! All web assembly indicies definitions.

use super::values::U32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TypeIdx(pub U32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FuncIdx(pub U32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TableIdx(pub U32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemIdx(pub U32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GlobalIdx(pub U32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ElemIdx(pub U32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DataIdx(pub U32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LocalIdx(pub U32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LabelIdx(pub U32);
