//! Web assembly types definitions.

use super::values::U32;

#[derive(Debug, PartialEq, Eq)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ResultType {
    pub types: Vec<ValType>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FuncType {
    pub params: ResultType,
    pub result: ResultType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Limits {
    pub min: U32,
    pub max: Option<U32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MemType {
    pub lim: Limits,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableType {
    pub lim: Limits,
    pub tpe: RefType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GlobalType {
    pub mutability: Mutability,
    pub tpe: ValType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mutability {
    Const,
    Mut,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExternType {
    Func(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
