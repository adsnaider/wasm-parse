use super::values::U32;

#[derive(Debug)]
pub struct TypeIdx(pub U32);
#[derive(Debug)]
pub struct FuncIdx(pub U32);
#[derive(Debug)]
pub struct TableIdx(pub U32);
#[derive(Debug)]
pub struct MemIdx(pub U32);
#[derive(Debug)]
pub struct GlobalIdx(pub U32);
#[derive(Debug)]
pub struct ElemIdx(pub U32);
#[derive(Debug)]
pub struct DataIdx(pub U32);
#[derive(Debug)]
pub struct LocalIdx(pub U32);
#[derive(Debug)]
pub struct LabelIdx(pub U32);
