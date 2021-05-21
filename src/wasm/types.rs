use super::values::U32;

#[derive(Debug)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(Debug)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

#[derive(Debug)]
pub struct ResultType {
    pub types: Vec<ValType>,
}

#[derive(Debug)]
pub struct FuncType {
    pub params: ResultType,
    pub result: ResultType,
}

#[derive(Debug)]
pub struct Limits {
    pub min: U32,
    pub max: Option<U32>,
}

#[derive(Debug)]
pub struct MemType {
    pub lim: Limits,
}

#[derive(Debug)]
pub struct TableType {
    pub lim: Limits,
    pub tpe: RefType,
}

#[derive(Debug)]
pub struct GlobalType {
    pub mutability: Mutability,
    pub tpe: ValType,
}

#[derive(Debug)]
pub enum Mutability {
    Const,
    Mut,
}

#[derive(Debug)]
pub enum ExternType {
    Func(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
