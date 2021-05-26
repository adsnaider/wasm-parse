//! Web assembly instruction and expression definitions.

use crate::wasm::indices::{
    DataIdx, ElemIdx, FuncIdx, GlobalIdx, LabelIdx, LocalIdx, TableIdx, TypeIdx,
};
use crate::wasm::types::{RefType, ValType};
use crate::wasm::values::U32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    Undefined,
    Numeric(NumericInstr),
    Reference(ReferenceInstr),
    Dropp,
    Select(Option<ValType>),
    Variable(VariableInstr),
    Table(TableInstr),
    Memory(MemoryInstr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntType {
    I32,
    I64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FloatType {
    F32,
    F64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sign {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableInstr {
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TableInstr {
    TableGet(TableIdx),
    TableSet(TableIdx),
    TableSize(TableIdx),
    TableGrow(TableIdx),
    TableFill(TableIdx),
    TableCopy(TableIdx, TableIdx),
    TableInit(TableIdx, ElemIdx),
    ElemDrop(ElemIdx),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MemArg {
    offset: U32,
    align: U32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemoryInstr {
    ILoad(IntType, MemArg),
    FLoad(IntType, MemArg),
    IStore(IntType, MemArg),
    FStore(IntType, MemArg),
    ILoad8(IntType, Sign, MemArg),
    ILoad16(IntType, Sign, MemArg),
    I64Load32(Sign, MemArg),
    IStore8(IntType, MemArg),
    IStore16(IntType, MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,
    MemoryFill,
    MemoryCopy,
    MemoryInit(DataIdx),
    DataDrop(DataIdx),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlockType {
    Type(TypeIdx),
    Val(Option<ValType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ControlInstr {
    Nop,
    Unreachable,
    Block(BlockType, Vec<Instr>),
    Loop(BlockType, Vec<Instr>),
    If(BlockType, Vec<Instr>, Option<Vec<Instr>>),
    Branch(LabelIdx),
    BranchIf(LabelIdx),
    BrancTable(Vec<LabelIdx>),
    Return,
    Call(FuncIdx),
    CallIndirect(TableIdx, TypeIdx),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReferenceInstr {
    RefNull(RefType),
    RefIsNull,
    RefFunc(FuncIdx),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NumericInstr {
    IConst(IntType),
    FConst(FloatType),
    IUnary(IntType, IUnop),
    FUnary(FloatType, FUnop),
    IBinary(IntType, IBinop),
    FBinary(FloatType, FBinop),
    ITest(IntType, ITestop),
    IRelop(IntType, IRelop),
    FRelop(FloatType, FRelop),
    IExtend8S(IntType),
    IExtend16S(IntType),
    I64Extend32S,
    I32WrapI64,
    I64ExtendI32(Sign),
    ITruncF(IntType, FloatType, Sign),
    ITrucSatF(IntType, FloatType, Sign),
    F32DemoteF64,
    F64PromoteF32,
    FConvertI(FloatType, IntType, Sign),
    IReinterpretF(IntType, FloatType),
    FReinterpretI(FloatType, IntType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IUnop {
    Clz,
    Ctz,
    Popcnt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FUnop {
    Abs,
    Neg,
    Sqrt,
    Ceil,
    Floor,
    Trunc,
    Nearest,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IBinop {
    Add,
    Sub,
    Mul,
    Div(Sign),
    Rem(Sign),
    And,
    Or,
    Xor,
    Shl,
    Shr(Sign),
    Rotl,
    Rotr,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FBinop {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    CopySign,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ITestop {
    Eqz,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IRelop {
    Equ,
    Ne,
    Lt(Sign),
    Gt(Sign),
    Le(Sign),
    Ge(Sign),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FRelop {
    Equ,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expr {
    pub instructions: Vec<Instr>,
}
