//! Web assembly instruction and expression definitions.

use crate::wasm::indices::{
    DataIdx, ElemIdx, FuncIdx, GlobalIdx, LabelIdx, LocalIdx, TableIdx, TypeIdx,
};
use crate::wasm::types::{RefType, ValType};
use crate::wasm::values::{F32, F64, I32, I64, U32};

#[derive(Debug, Clone)]
pub enum Instr {
    Numeric(NumericInstr),
    Reference(ReferenceInstr),
    Dropp,
    Select(Vec<ValType>),
    Variable(VariableInstr),
    Table(TableInstr),
    Memory(MemoryInstr),
    Control(ControlInstr),
}

#[derive(Debug, Clone)]
pub enum IntType {
    I32,
    I64,
}

#[derive(Debug, Clone)]
pub enum FloatType {
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub enum Sign {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone)]
pub enum VariableInstr {
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct MemArg {
    pub offset: U32,
    pub align: U32,
}

#[derive(Debug, Clone)]
pub enum MemoryInstr {
    ILoad(IntType, MemArg),
    FLoad(FloatType, MemArg),
    IStore(IntType, MemArg),
    FStore(FloatType, MemArg),
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

#[derive(Debug, Clone)]
pub enum BlockType {
    Type(TypeIdx),
    Val(ValType),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub tpe: BlockType,
    pub instr: Vec<Instr>,
}

#[derive(Debug, Clone)]
pub struct IfElseBlock {
    pub tpe: BlockType,
    pub if_br: Vec<Instr>,
    pub else_br: Vec<Instr>,
}

#[derive(Debug, Clone)]
pub enum ControlInstr {
    Nop,
    Unreachable,
    Block(Block),
    Loop(Block),
    If(IfElseBlock),
    Branch(LabelIdx),
    BranchIf(LabelIdx),
    BrancTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TableIdx, TypeIdx),
}

#[derive(Debug, Clone)]
pub enum ReferenceInstr {
    RefNull(RefType),
    RefIsNull,
    RefFunc(FuncIdx),
}

#[derive(Debug, Clone)]
pub enum NumericInstr {
    I32Const(I32),
    I64Const(I64),
    F32Const(F32),
    F64Const(F64),
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
    ITruncSatF(IntType, FloatType, Sign),
    F32DemoteF64,
    F64PromoteF32,
    FConvertI(FloatType, IntType, Sign),
    IReinterpretF(IntType, FloatType),
    FReinterpretI(FloatType, IntType),
}

#[derive(Debug, Clone)]
pub enum IUnop {
    Clz,
    Ctz,
    Popcnt,
}

#[derive(Debug, Clone)]
pub enum FUnop {
    Abs,
    Neg,
    Sqrt,
    Ceil,
    Floor,
    Trunc,
    Nearest,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum FBinop {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    CopySign,
}

#[derive(Debug, Clone)]
pub enum ITestop {
    Eqz,
}

#[derive(Debug, Clone)]
pub enum IRelop {
    Equ,
    Ne,
    Lt(Sign),
    Gt(Sign),
    Le(Sign),
    Ge(Sign),
}

#[derive(Debug, Clone)]
pub enum FRelop {
    Equ,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub instr: Vec<Instr>,
}
