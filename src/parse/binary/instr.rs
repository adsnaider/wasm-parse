use super::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::indices::{
    DataIdx, ElemIdx, FuncIdx, GlobalIdx, LabelIdx, LocalIdx, TableIdx, TypeIdx,
};
use crate::wasm::instr::{
    Block, BlockType, ControlInstr, Expr, FBinop, FRelop, FUnop, FloatType, IBinop, IRelop,
    ITestop, IUnop, IfElseBlock, Instr, IntType, MemArg, MemoryInstr, NumericInstr, ReferenceInstr,
    Sign, TableInstr, VariableInstr,
};
use crate::wasm::types::{RefType, ValType};
use crate::wasm::values::{F32, F64, I32, I64, S64, U32};
use std::convert::TryInto;

// TODO: This
impl Parse for Instr {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't parse instruction. Buffer too small.".to_string(),
            ));
        }
        let instr = match data.consume(()) {
            0x00 => Instr::Control(ControlInstr::Unreachable),
            0x01 => Instr::Control(ControlInstr::Nop),
            0x02 => Instr::Control(ControlInstr::Block(Block::parse(data)?)),
            0x03 => Instr::Control(ControlInstr::Loop(Block::parse(data)?)),
            0x04 => Instr::Control(ControlInstr::If(IfElseBlock::parse(data)?)),
            0x0C => Instr::Control(ControlInstr::Branch(LabelIdx::parse(data)?)),
            0x0D => Instr::Control(ControlInstr::BranchIf(LabelIdx::parse(data)?)),
            0x0E => {
                let labels = Vec::parse(data)?;
                let label = LabelIdx::parse(data)?;
                Instr::Control(ControlInstr::BrancTable(labels, label))
            }
            0x0F => Instr::Control(ControlInstr::Return),
            0x10 => Instr::Control(ControlInstr::Call(FuncIdx::parse(data)?)),
            0x11 => {
                let y = TypeIdx::parse(data)?;
                let x = TableIdx::parse(data)?;
                Instr::Control(ControlInstr::CallIndirect(x, y))
            }
            0xD0 => Instr::Reference(ReferenceInstr::RefNull(RefType::parse(data)?)),
            0xD1 => Instr::Reference(ReferenceInstr::RefIsNull),
            0xD2 => Instr::Reference(ReferenceInstr::RefFunc(FuncIdx::parse(data)?)),
            0x1A => Instr::Dropp,
            0x1B => Instr::Select(Vec::new()),
            0x1C => Instr::Select(Vec::parse(data)?),
            0x20 => Instr::Variable(VariableInstr::LocalGet(LocalIdx::parse(data)?)),
            0x21 => Instr::Variable(VariableInstr::LocalSet(LocalIdx::parse(data)?)),
            0x22 => Instr::Variable(VariableInstr::LocalTee(LocalIdx::parse(data)?)),
            0x23 => Instr::Variable(VariableInstr::GlobalGet(GlobalIdx::parse(data)?)),
            0x24 => Instr::Variable(VariableInstr::GlobalSet(GlobalIdx::parse(data)?)),
            0x25 => Instr::Table(TableInstr::TableGet(TableIdx::parse(data)?)),
            0x26 => Instr::Table(TableInstr::TableSet(TableIdx::parse(data)?)),
            0xFC => {
                let selector = *U32::parse(data)?;
                match selector {
                    // Trunc saturated
                    0 => Instr::Numeric(NumericInstr::ITruncSatF(
                        IntType::I32,
                        FloatType::F32,
                        Sign::Signed,
                    )),
                    1 => Instr::Numeric(NumericInstr::ITruncSatF(
                        IntType::I32,
                        FloatType::F32,
                        Sign::Unsigned,
                    )),
                    2 => Instr::Numeric(NumericInstr::ITruncSatF(
                        IntType::I32,
                        FloatType::F64,
                        Sign::Signed,
                    )),
                    3 => Instr::Numeric(NumericInstr::ITruncSatF(
                        IntType::I32,
                        FloatType::F64,
                        Sign::Unsigned,
                    )),
                    4 => Instr::Numeric(NumericInstr::ITruncSatF(
                        IntType::I64,
                        FloatType::F32,
                        Sign::Signed,
                    )),
                    5 => Instr::Numeric(NumericInstr::ITruncSatF(
                        IntType::I64,
                        FloatType::F32,
                        Sign::Unsigned,
                    )),
                    6 => Instr::Numeric(NumericInstr::ITruncSatF(
                        IntType::I64,
                        FloatType::F64,
                        Sign::Signed,
                    )),
                    7 => Instr::Numeric(NumericInstr::ITruncSatF(
                        IntType::I64,
                        FloatType::F64,
                        Sign::Unsigned,
                    )),

                    // Mem instructions:
                    8 => {
                        let x = DataIdx::parse(data)?;
                        if data.consume(()) != 0x00 {
                            return Err(ParseError::new(
                                data,
                                "Invalid memory size instruction. Second byte should be 0x00"
                                    .to_string(),
                            ));
                        }
                        Instr::Memory(MemoryInstr::MemoryInit(x))
                    }
                    9 => {
                        let x = DataIdx::parse(data)?;
                        Instr::Memory(MemoryInstr::DataDrop(x))
                    }
                    10 => {
                        if *data.consume(2) != [0x00, 0x00] {
                            return Err(ParseError::new(
                                data,
                                "Invalid memory copy instruction. Proceeding 2 bytes should be 0x00"
                                    .to_string(),
                            ));
                        }
                        Instr::Memory(MemoryInstr::MemoryCopy)
                    }
                    11 => {
                        if data.consume(()) != 0x00 {
                            return Err(ParseError::new(
                                data,
                                "Invalid memory fill instruction. Second byte should be 0x00"
                                    .to_string(),
                            ));
                        }
                        Instr::Memory(MemoryInstr::MemoryFill)
                    }

                    // Table instructions
                    12 => {
                        let y = ElemIdx::parse(data)?;
                        let x = TableIdx::parse(data)?;
                        Instr::Table(TableInstr::TableInit(x, y))
                    }
                    13 => Instr::Table(TableInstr::ElemDrop(ElemIdx::parse(data)?)),
                    14 => {
                        let x = TableIdx::parse(data)?;
                        let y = TableIdx::parse(data)?;
                        Instr::Table(TableInstr::TableCopy(x, y))
                    }
                    15 => Instr::Table(TableInstr::TableGrow(TableIdx::parse(data)?)),
                    16 => Instr::Table(TableInstr::TableSize(TableIdx::parse(data)?)),
                    17 => Instr::Table(TableInstr::TableFill(TableIdx::parse(data)?)),

                    x => {
                        return Err(ParseError::new(
                            data,
                            format!("Unknown 0xFC instruction {}", x),
                        ))
                    }
                }
            }
            0x28 => Instr::Memory(MemoryInstr::ILoad(IntType::I32, MemArg::parse(data)?)),
            0x29 => Instr::Memory(MemoryInstr::ILoad(IntType::I64, MemArg::parse(data)?)),
            0x2A => Instr::Memory(MemoryInstr::FLoad(FloatType::F32, MemArg::parse(data)?)),
            0x2B => Instr::Memory(MemoryInstr::FLoad(FloatType::F64, MemArg::parse(data)?)),
            0x2C => Instr::Memory(MemoryInstr::ILoad8(
                IntType::I32,
                Sign::Signed,
                MemArg::parse(data)?,
            )),
            0x2D => Instr::Memory(MemoryInstr::ILoad8(
                IntType::I32,
                Sign::Unsigned,
                MemArg::parse(data)?,
            )),
            0x2E => Instr::Memory(MemoryInstr::ILoad16(
                IntType::I32,
                Sign::Signed,
                MemArg::parse(data)?,
            )),
            0x2F => Instr::Memory(MemoryInstr::ILoad16(
                IntType::I32,
                Sign::Unsigned,
                MemArg::parse(data)?,
            )),
            0x30 => Instr::Memory(MemoryInstr::ILoad8(
                IntType::I64,
                Sign::Signed,
                MemArg::parse(data)?,
            )),
            0x31 => Instr::Memory(MemoryInstr::ILoad8(
                IntType::I64,
                Sign::Unsigned,
                MemArg::parse(data)?,
            )),
            0x32 => Instr::Memory(MemoryInstr::ILoad16(
                IntType::I64,
                Sign::Signed,
                MemArg::parse(data)?,
            )),
            0x33 => Instr::Memory(MemoryInstr::ILoad16(
                IntType::I64,
                Sign::Unsigned,
                MemArg::parse(data)?,
            )),
            0x34 => Instr::Memory(MemoryInstr::I64Load32(Sign::Signed, MemArg::parse(data)?)),
            0x35 => Instr::Memory(MemoryInstr::I64Load32(Sign::Unsigned, MemArg::parse(data)?)),
            0x36 => Instr::Memory(MemoryInstr::IStore(IntType::I32, MemArg::parse(data)?)),
            0x37 => Instr::Memory(MemoryInstr::IStore(IntType::I64, MemArg::parse(data)?)),
            0x38 => Instr::Memory(MemoryInstr::FStore(FloatType::F32, MemArg::parse(data)?)),
            0x39 => Instr::Memory(MemoryInstr::FStore(FloatType::F64, MemArg::parse(data)?)),
            0x3A => Instr::Memory(MemoryInstr::IStore8(IntType::I32, MemArg::parse(data)?)),
            0x3B => Instr::Memory(MemoryInstr::IStore16(IntType::I32, MemArg::parse(data)?)),
            0x3C => Instr::Memory(MemoryInstr::IStore8(IntType::I64, MemArg::parse(data)?)),
            0x3D => Instr::Memory(MemoryInstr::IStore16(IntType::I64, MemArg::parse(data)?)),
            0x3E => Instr::Memory(MemoryInstr::I64Store32(MemArg::parse(data)?)),
            0x3F => {
                if data.consume(()) != 0x00 {
                    return Err(ParseError::new(
                        data,
                        "Invalid memory size instruction. Second byte should be 0x00".to_string(),
                    ));
                }
                Instr::Memory(MemoryInstr::MemorySize)
            }
            0x40 => {
                if data.consume(()) != 0x00 {
                    return Err(ParseError::new(
                        data,
                        "Invalid memory grow instruction. Second byte should be 0x00".to_string(),
                    ));
                }
                Instr::Memory(MemoryInstr::MemoryGrow)
            }
            0x41 => Instr::Numeric(NumericInstr::I32Const(I32::parse(data)?)),
            0x42 => Instr::Numeric(NumericInstr::I64Const(I64::parse(data)?)),
            0x43 => Instr::Numeric(NumericInstr::F32Const(F32::parse(data)?)),
            0x44 => Instr::Numeric(NumericInstr::F64Const(F64::parse(data)?)),
            0x45 => Instr::Numeric(NumericInstr::ITest(IntType::I32, ITestop::Eqz)),
            0x46 => Instr::Numeric(NumericInstr::IRelop(IntType::I32, IRelop::Equ)),
            0x47 => Instr::Numeric(NumericInstr::IRelop(IntType::I32, IRelop::Ne)),
            0x48 => Instr::Numeric(NumericInstr::IRelop(IntType::I32, IRelop::Lt(Sign::Signed))),
            0x49 => Instr::Numeric(NumericInstr::IRelop(
                IntType::I32,
                IRelop::Lt(Sign::Unsigned),
            )),
            0x4A => Instr::Numeric(NumericInstr::IRelop(IntType::I32, IRelop::Gt(Sign::Signed))),
            0x4B => Instr::Numeric(NumericInstr::IRelop(
                IntType::I32,
                IRelop::Gt(Sign::Unsigned),
            )),
            0x4C => Instr::Numeric(NumericInstr::IRelop(IntType::I32, IRelop::Le(Sign::Signed))),
            0x4D => Instr::Numeric(NumericInstr::IRelop(
                IntType::I32,
                IRelop::Le(Sign::Unsigned),
            )),
            0x4E => Instr::Numeric(NumericInstr::IRelop(IntType::I32, IRelop::Ge(Sign::Signed))),
            0x4F => Instr::Numeric(NumericInstr::IRelop(
                IntType::I32,
                IRelop::Ge(Sign::Unsigned),
            )),
            0x50 => Instr::Numeric(NumericInstr::ITest(IntType::I64, ITestop::Eqz)),
            0x51 => Instr::Numeric(NumericInstr::IRelop(IntType::I64, IRelop::Equ)),
            0x52 => Instr::Numeric(NumericInstr::IRelop(IntType::I64, IRelop::Ne)),
            0x53 => Instr::Numeric(NumericInstr::IRelop(IntType::I64, IRelop::Lt(Sign::Signed))),
            0x54 => Instr::Numeric(NumericInstr::IRelop(
                IntType::I64,
                IRelop::Lt(Sign::Unsigned),
            )),
            0x55 => Instr::Numeric(NumericInstr::IRelop(IntType::I64, IRelop::Gt(Sign::Signed))),
            0x56 => Instr::Numeric(NumericInstr::IRelop(
                IntType::I64,
                IRelop::Gt(Sign::Unsigned),
            )),
            0x57 => Instr::Numeric(NumericInstr::IRelop(IntType::I64, IRelop::Le(Sign::Signed))),
            0x58 => Instr::Numeric(NumericInstr::IRelop(
                IntType::I64,
                IRelop::Le(Sign::Unsigned),
            )),
            0x59 => Instr::Numeric(NumericInstr::IRelop(IntType::I64, IRelop::Ge(Sign::Signed))),
            0x5A => Instr::Numeric(NumericInstr::IRelop(
                IntType::I64,
                IRelop::Ge(Sign::Unsigned),
            )),
            0x5B => Instr::Numeric(NumericInstr::FRelop(FloatType::F32, FRelop::Equ)),
            0x5C => Instr::Numeric(NumericInstr::FRelop(FloatType::F32, FRelop::Ne)),
            0x5D => Instr::Numeric(NumericInstr::FRelop(FloatType::F32, FRelop::Lt)),
            0x5E => Instr::Numeric(NumericInstr::FRelop(FloatType::F32, FRelop::Gt)),
            0x5F => Instr::Numeric(NumericInstr::FRelop(FloatType::F32, FRelop::Le)),
            0x60 => Instr::Numeric(NumericInstr::FRelop(FloatType::F32, FRelop::Ge)),
            0x61 => Instr::Numeric(NumericInstr::FRelop(FloatType::F64, FRelop::Equ)),
            0x62 => Instr::Numeric(NumericInstr::FRelop(FloatType::F64, FRelop::Ne)),
            0x63 => Instr::Numeric(NumericInstr::FRelop(FloatType::F64, FRelop::Lt)),
            0x64 => Instr::Numeric(NumericInstr::FRelop(FloatType::F64, FRelop::Gt)),
            0x65 => Instr::Numeric(NumericInstr::FRelop(FloatType::F64, FRelop::Le)),
            0x66 => Instr::Numeric(NumericInstr::FRelop(FloatType::F64, FRelop::Ge)),

            0x67 => Instr::Numeric(NumericInstr::IUnary(IntType::I32, IUnop::Clz)),
            0x68 => Instr::Numeric(NumericInstr::IUnary(IntType::I32, IUnop::Ctz)),
            0x69 => Instr::Numeric(NumericInstr::IUnary(IntType::I32, IUnop::Popcnt)),
            0x6A => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::Add)),
            0x6B => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::Sub)),
            0x6C => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::Mul)),
            0x6D => Instr::Numeric(NumericInstr::IBinary(
                IntType::I32,
                IBinop::Div(Sign::Signed),
            )),
            0x6E => Instr::Numeric(NumericInstr::IBinary(
                IntType::I32,
                IBinop::Div(Sign::Unsigned),
            )),
            0x6F => Instr::Numeric(NumericInstr::IBinary(
                IntType::I32,
                IBinop::Rem(Sign::Signed),
            )),
            0x70 => Instr::Numeric(NumericInstr::IBinary(
                IntType::I32,
                IBinop::Rem(Sign::Unsigned),
            )),
            0x71 => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::And)),
            0x72 => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::Or)),
            0x73 => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::Xor)),
            0x74 => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::Shl)),
            0x75 => Instr::Numeric(NumericInstr::IBinary(
                IntType::I32,
                IBinop::Shr(Sign::Signed),
            )),
            0x76 => Instr::Numeric(NumericInstr::IBinary(
                IntType::I32,
                IBinop::Shr(Sign::Unsigned),
            )),
            0x77 => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::Rotl)),
            0x78 => Instr::Numeric(NumericInstr::IBinary(IntType::I32, IBinop::Rotr)),

            0x79 => Instr::Numeric(NumericInstr::IUnary(IntType::I64, IUnop::Clz)),
            0x7A => Instr::Numeric(NumericInstr::IUnary(IntType::I64, IUnop::Ctz)),
            0x7B => Instr::Numeric(NumericInstr::IUnary(IntType::I64, IUnop::Popcnt)),
            0x7C => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::Add)),
            0x7D => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::Sub)),
            0x7E => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::Mul)),
            0x7F => Instr::Numeric(NumericInstr::IBinary(
                IntType::I64,
                IBinop::Div(Sign::Signed),
            )),
            0x80 => Instr::Numeric(NumericInstr::IBinary(
                IntType::I64,
                IBinop::Div(Sign::Unsigned),
            )),
            0x81 => Instr::Numeric(NumericInstr::IBinary(
                IntType::I64,
                IBinop::Rem(Sign::Signed),
            )),
            0x82 => Instr::Numeric(NumericInstr::IBinary(
                IntType::I64,
                IBinop::Rem(Sign::Unsigned),
            )),
            0x83 => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::And)),
            0x84 => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::Or)),
            0x85 => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::Xor)),
            0x86 => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::Shl)),
            0x87 => Instr::Numeric(NumericInstr::IBinary(
                IntType::I64,
                IBinop::Shr(Sign::Signed),
            )),
            0x88 => Instr::Numeric(NumericInstr::IBinary(
                IntType::I64,
                IBinop::Shr(Sign::Unsigned),
            )),
            0x89 => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::Rotl)),
            0x8A => Instr::Numeric(NumericInstr::IBinary(IntType::I64, IBinop::Rotr)),

            0x8B => Instr::Numeric(NumericInstr::FUnary(FloatType::F32, FUnop::Abs)),
            0x8C => Instr::Numeric(NumericInstr::FUnary(FloatType::F32, FUnop::Neg)),
            0x8D => Instr::Numeric(NumericInstr::FUnary(FloatType::F32, FUnop::Ceil)),
            0x8E => Instr::Numeric(NumericInstr::FUnary(FloatType::F32, FUnop::Floor)),
            0x8F => Instr::Numeric(NumericInstr::FUnary(FloatType::F32, FUnop::Trunc)),
            0x90 => Instr::Numeric(NumericInstr::FUnary(FloatType::F32, FUnop::Nearest)),
            0x91 => Instr::Numeric(NumericInstr::FUnary(FloatType::F32, FUnop::Sqrt)),
            0x92 => Instr::Numeric(NumericInstr::FBinary(FloatType::F32, FBinop::Add)),
            0x93 => Instr::Numeric(NumericInstr::FBinary(FloatType::F32, FBinop::Sub)),
            0x94 => Instr::Numeric(NumericInstr::FBinary(FloatType::F32, FBinop::Mul)),
            0x95 => Instr::Numeric(NumericInstr::FBinary(FloatType::F32, FBinop::Div)),
            0x96 => Instr::Numeric(NumericInstr::FBinary(FloatType::F32, FBinop::Min)),
            0x97 => Instr::Numeric(NumericInstr::FBinary(FloatType::F32, FBinop::Max)),
            0x98 => Instr::Numeric(NumericInstr::FBinary(FloatType::F32, FBinop::CopySign)),

            0x99 => Instr::Numeric(NumericInstr::FUnary(FloatType::F64, FUnop::Abs)),
            0x9A => Instr::Numeric(NumericInstr::FUnary(FloatType::F64, FUnop::Neg)),
            0x9B => Instr::Numeric(NumericInstr::FUnary(FloatType::F64, FUnop::Ceil)),
            0x9C => Instr::Numeric(NumericInstr::FUnary(FloatType::F64, FUnop::Floor)),
            0x9D => Instr::Numeric(NumericInstr::FUnary(FloatType::F64, FUnop::Trunc)),
            0x9E => Instr::Numeric(NumericInstr::FUnary(FloatType::F64, FUnop::Nearest)),
            0x9F => Instr::Numeric(NumericInstr::FUnary(FloatType::F64, FUnop::Sqrt)),
            0xA0 => Instr::Numeric(NumericInstr::FBinary(FloatType::F64, FBinop::Add)),
            0xA1 => Instr::Numeric(NumericInstr::FBinary(FloatType::F64, FBinop::Sub)),
            0xA2 => Instr::Numeric(NumericInstr::FBinary(FloatType::F64, FBinop::Mul)),
            0xA3 => Instr::Numeric(NumericInstr::FBinary(FloatType::F64, FBinop::Div)),
            0xA4 => Instr::Numeric(NumericInstr::FBinary(FloatType::F64, FBinop::Min)),
            0xA5 => Instr::Numeric(NumericInstr::FBinary(FloatType::F64, FBinop::Max)),
            0xA6 => Instr::Numeric(NumericInstr::FBinary(FloatType::F64, FBinop::CopySign)),

            0xA7 => Instr::Numeric(NumericInstr::I32WrapI64),
            0xA8 => Instr::Numeric(NumericInstr::ITruncF(
                IntType::I32,
                FloatType::F32,
                Sign::Signed,
            )),
            0xA9 => Instr::Numeric(NumericInstr::ITruncF(
                IntType::I32,
                FloatType::F32,
                Sign::Unsigned,
            )),
            0xAA => Instr::Numeric(NumericInstr::ITruncF(
                IntType::I32,
                FloatType::F64,
                Sign::Signed,
            )),
            0xAB => Instr::Numeric(NumericInstr::ITruncF(
                IntType::I32,
                FloatType::F64,
                Sign::Unsigned,
            )),
            0xAC => Instr::Numeric(NumericInstr::I64ExtendI32(Sign::Signed)),
            0xAD => Instr::Numeric(NumericInstr::I64ExtendI32(Sign::Unsigned)),
            0xAE => Instr::Numeric(NumericInstr::ITruncF(
                IntType::I64,
                FloatType::F32,
                Sign::Signed,
            )),
            0xAF => Instr::Numeric(NumericInstr::ITruncF(
                IntType::I64,
                FloatType::F32,
                Sign::Unsigned,
            )),
            0xB0 => Instr::Numeric(NumericInstr::ITruncF(
                IntType::I64,
                FloatType::F64,
                Sign::Signed,
            )),
            0xB1 => Instr::Numeric(NumericInstr::ITruncF(
                IntType::I64,
                FloatType::F64,
                Sign::Unsigned,
            )),
            0xB2 => Instr::Numeric(NumericInstr::FConvertI(
                FloatType::F32,
                IntType::I32,
                Sign::Signed,
            )),
            0xB3 => Instr::Numeric(NumericInstr::FConvertI(
                FloatType::F32,
                IntType::I32,
                Sign::Unsigned,
            )),
            0xB4 => Instr::Numeric(NumericInstr::FConvertI(
                FloatType::F32,
                IntType::I64,
                Sign::Signed,
            )),
            0xB5 => Instr::Numeric(NumericInstr::FConvertI(
                FloatType::F32,
                IntType::I64,
                Sign::Unsigned,
            )),
            0xB6 => Instr::Numeric(NumericInstr::F32DemoteF64),
            0xB7 => Instr::Numeric(NumericInstr::FConvertI(
                FloatType::F64,
                IntType::I32,
                Sign::Signed,
            )),
            0xB8 => Instr::Numeric(NumericInstr::FConvertI(
                FloatType::F64,
                IntType::I32,
                Sign::Unsigned,
            )),
            0xB9 => Instr::Numeric(NumericInstr::FConvertI(
                FloatType::F64,
                IntType::I64,
                Sign::Signed,
            )),
            0xBA => Instr::Numeric(NumericInstr::FConvertI(
                FloatType::F64,
                IntType::I64,
                Sign::Unsigned,
            )),
            0xBB => Instr::Numeric(NumericInstr::F64PromoteF32),
            0xBC => Instr::Numeric(NumericInstr::IReinterpretF(IntType::I32, FloatType::F32)),
            0xBD => Instr::Numeric(NumericInstr::IReinterpretF(IntType::I64, FloatType::F64)),
            0xBE => Instr::Numeric(NumericInstr::FReinterpretI(FloatType::F32, IntType::I32)),
            0xBF => Instr::Numeric(NumericInstr::FReinterpretI(FloatType::F64, IntType::I64)),

            0xC0 => Instr::Numeric(NumericInstr::IExtend8S(IntType::I32)),
            0xC1 => Instr::Numeric(NumericInstr::IExtend16S(IntType::I32)),
            0xC2 => Instr::Numeric(NumericInstr::IExtend8S(IntType::I64)),
            0xC3 => Instr::Numeric(NumericInstr::IExtend16S(IntType::I64)),
            0xC4 => Instr::Numeric(NumericInstr::I64Extend32S),

            x => {
                return Err(ParseError::new(
                    data,
                    format!("Unknown instruction number {:X}", x),
                ));
            }
        };

        Ok(instr)
    }
}

impl Parse for Expr {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let mut instr = Vec::new();
        while data.read(()) != 0x0B {
            instr.push(Instr::parse(data)?);
        }
        data.consume(());

        Ok(Expr { instr })
    }
}

impl Parse for MemArg {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let align = U32::parse(data)?;
        let offset = U32::parse(data)?;
        Ok(MemArg { align, offset })
    }
}

impl Parse for Block {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = BlockType::parse(data)?;
        let mut instr = Vec::new();
        while data.read(()) != 0x0B {
            instr.push(Instr::parse(data)?);
        }
        data.consume(());
        Ok(Block { tpe, instr })
    }
}

impl Parse for IfElseBlock {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let tpe = BlockType::parse(data)?;
        let mut if_br = Vec::new();
        let mut else_br = Vec::new();
        loop {
            match data.read(()) {
                0x0B => {
                    data.consume(());
                    return Ok(IfElseBlock {
                        tpe,
                        if_br,
                        else_br,
                    });
                }
                0x05 => {
                    data.consume(());
                    break;
                }
                _ => {
                    if_br.push(Instr::parse(data)?);
                }
            }
        }

        loop {
            match data.read(()) {
                0x0B => {
                    data.consume(());
                    return Ok(IfElseBlock {
                        tpe,
                        if_br,
                        else_br,
                    });
                }
                _ => {
                    else_br.push(Instr::parse(data)?);
                }
            }
        }
    }
}

impl Parse for BlockType {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let blocktype = match data.read(()) {
            0x40 => {
                data.consume(());
                BlockType::Empty
            }
            0x7C..=0x7F | 0x6F | 0x70 => {
                data.consume(());
                BlockType::Val(ValType::parse(data)?)
            }
            _ => {
                let val = *S64::parse(data)?;
                let val: u32 = val.try_into().or(Err(ParseError::new(
                    data,
                    format!("Can't decode block type. Invalid type index {}", val),
                )))?;
                BlockType::Type(TypeIdx(U32(val)))
            }
        };
        Ok(blocktype)
    }
}
