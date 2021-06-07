//! Web assembly types definitions.

use super::values::U32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResultType {
    pub types: Vec<ValType>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncType {
    pub params: ResultType,
    pub result: ResultType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Limits {
    pub min: U32,
    pub max: Option<U32>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MemType {
    pub lim: Limits,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TableType {
    pub lim: Limits,
    pub tpe: RefType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct GlobalType {
    pub mutability: Mutability,
    pub tpe: ValType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mutability {
    Const,
    Mut,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExternType {
    Func(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
