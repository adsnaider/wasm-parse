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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IntType {
    I32 = 0,
    I64 = 1,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FloatType {
    F32,
    F64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
    BranchTable(Vec<LabelIdx>, LabelIdx),
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
    I32Unary(IUnop<{ IntType::I32 }>),
    I64Unary(IUnop<{ IntType::I64 }>),
    F32Unary(FUnop<{ FloatType::F32 }>),
    F64Unary(FUnop<{ FloatType::F64 }>),
    I32Binary(IBinop<{ IntType::I32 }>),
    I64Binary(IBinop<{ IntType::I64 }>),
    F32Binary(FBinop<{ FloatType::F32 }>),
    F64Binary(FBinop<{ FloatType::F64 }>),
    I32Test(ITestop<{ IntType::I32 }>),
    I64Test(ITestop<{ IntType::I64 }>),
    I32Relop(IRelop<{ IntType::I32 }>),
    I64Relop(IRelop<{ IntType::I64 }>),
    F32Relop(FRelop<{ FloatType::F32 }>),
    F64Relop(FRelop<{ FloatType::F64 }>),
    I32Extend8S,
    I64Extend8S,
    I32Extend16S,
    I64Extend16S,
    I64Extend32S,
    I32WrapI64,
    I64ExtendI32S,
    I64ExtendI32U,
    I32TruncF32S,
    I32TruncF32U,
    I64TruncF32S,
    I64TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64TruncF64S,
    I64TruncF64U,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
    F32DemoteF64,
    F64PromoteF32,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}

#[derive(Debug, Clone)]
pub enum IUnop<const T: IntType> {
    Clz,
    Ctz,
    Popcnt,
}

#[derive(Debug, Clone)]
pub enum FUnop<const T: FloatType> {
    Abs,
    Neg,
    Sqrt,
    Ceil,
    Floor,
    Trunc,
    Nearest,
}

#[derive(Debug, Clone)]
pub enum IBinop<const T: IntType> {
    Add,
    Sub,
    Mul,
    DivS,
    DivU,
    RemS,
    RemU,
    And,
    Or,
    Xor,
    Shl,
    ShrS,
    ShrU,
    Rotl,
    Rotr,
}

#[derive(Debug, Clone)]
pub enum FBinop<const T: FloatType> {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    CopySign,
}

#[derive(Debug, Clone)]
pub enum ITestop<const T: IntType> {
    Eqz,
}

#[derive(Debug, Clone)]
pub enum IRelop<const T: IntType> {
    Equ,
    Ne,
    LtS,
    LtU,
    GtS,
    GtU,
    LeS,
    LeU,
    GeS,
    GeU,
}

#[derive(Debug, Clone)]
pub enum FRelop<const T: FloatType> {
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
