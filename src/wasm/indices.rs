//! All web assembly indicies definitions.

use super::values::U32;

#[derive(Debug, Copy, Clone)]
pub struct TypeIdx(pub U32);
#[derive(Debug, Copy, Clone)]
pub struct FuncIdx(pub U32);
#[derive(Debug, Copy, Clone)]
pub struct TableIdx(pub U32);
#[derive(Debug, Copy, Clone)]
pub struct MemIdx(pub U32);
#[derive(Debug, Copy, Clone)]
pub struct GlobalIdx(pub U32);
#[derive(Debug, Copy, Clone)]
pub struct ElemIdx(pub U32);
#[derive(Debug, Copy, Clone)]
pub struct DataIdx(pub U32);
#[derive(Debug, Copy, Clone)]
pub struct LocalIdx(pub U32);
#[derive(Debug, Copy, Clone)]
pub struct LabelIdx(pub U32);
