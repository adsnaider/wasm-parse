use std::convert::TryInto;

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
                Instr::Control(ControlInstr::BranchTable(labels, label))
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
                    0 => Instr::Numeric(NumericInstr::I32TruncSatF32S),
                    1 => Instr::Numeric(NumericInstr::I32TruncSatF32U),
                    2 => Instr::Numeric(NumericInstr::I32TruncSatF64S),
                    3 => Instr::Numeric(NumericInstr::I32TruncSatF64U),
                    4 => Instr::Numeric(NumericInstr::I64TruncSatF32S),
                    5 => Instr::Numeric(NumericInstr::I64TruncSatF32U),
                    6 => Instr::Numeric(NumericInstr::I64TruncSatF64S),
                    7 => Instr::Numeric(NumericInstr::I64TruncSatF64U),

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
            0x45 => Instr::Numeric(NumericInstr::I32Test(ITestop::Eqz)),
            0x46 => Instr::Numeric(NumericInstr::I32Relop(IRelop::Equ)),
            0x47 => Instr::Numeric(NumericInstr::I32Relop(IRelop::Ne)),
            0x48 => Instr::Numeric(NumericInstr::I32Relop(IRelop::LtS)),
            0x49 => Instr::Numeric(NumericInstr::I32Relop(IRelop::LtU)),
            0x4A => Instr::Numeric(NumericInstr::I32Relop(IRelop::GtS)),
            0x4B => Instr::Numeric(NumericInstr::I32Relop(IRelop::GtU)),
            0x4C => Instr::Numeric(NumericInstr::I32Relop(IRelop::LeS)),
            0x4D => Instr::Numeric(NumericInstr::I32Relop(IRelop::LeU)),
            0x4E => Instr::Numeric(NumericInstr::I32Relop(IRelop::GeS)),
            0x4F => Instr::Numeric(NumericInstr::I32Relop(IRelop::GeU)),
            0x50 => Instr::Numeric(NumericInstr::I64Test(ITestop::Eqz)),
            0x51 => Instr::Numeric(NumericInstr::I64Relop(IRelop::Equ)),
            0x52 => Instr::Numeric(NumericInstr::I64Relop(IRelop::Ne)),
            0x53 => Instr::Numeric(NumericInstr::I64Relop(IRelop::LtS)),
            0x54 => Instr::Numeric(NumericInstr::I64Relop(IRelop::LtU)),
            0x55 => Instr::Numeric(NumericInstr::I64Relop(IRelop::GtS)),
            0x56 => Instr::Numeric(NumericInstr::I64Relop(IRelop::GtU)),
            0x57 => Instr::Numeric(NumericInstr::I64Relop(IRelop::LeS)),
            0x58 => Instr::Numeric(NumericInstr::I64Relop(IRelop::LeU)),
            0x59 => Instr::Numeric(NumericInstr::I64Relop(IRelop::GeS)),
            0x5A => Instr::Numeric(NumericInstr::I64Relop(IRelop::GeU)),
            0x5B => Instr::Numeric(NumericInstr::F32Relop(FRelop::Equ)),
            0x5C => Instr::Numeric(NumericInstr::F32Relop(FRelop::Ne)),
            0x5D => Instr::Numeric(NumericInstr::F32Relop(FRelop::Lt)),
            0x5E => Instr::Numeric(NumericInstr::F32Relop(FRelop::Gt)),
            0x5F => Instr::Numeric(NumericInstr::F32Relop(FRelop::Le)),
            0x60 => Instr::Numeric(NumericInstr::F32Relop(FRelop::Ge)),
            0x61 => Instr::Numeric(NumericInstr::F32Relop(FRelop::Equ)),
            0x62 => Instr::Numeric(NumericInstr::F32Relop(FRelop::Ne)),
            0x63 => Instr::Numeric(NumericInstr::F32Relop(FRelop::Lt)),
            0x64 => Instr::Numeric(NumericInstr::F32Relop(FRelop::Gt)),
            0x65 => Instr::Numeric(NumericInstr::F32Relop(FRelop::Le)),
            0x66 => Instr::Numeric(NumericInstr::F32Relop(FRelop::Ge)),

            0x67 => Instr::Numeric(NumericInstr::I32Unary(IUnop::Clz)),
            0x68 => Instr::Numeric(NumericInstr::I32Unary(IUnop::Ctz)),
            0x69 => Instr::Numeric(NumericInstr::I32Unary(IUnop::Popcnt)),
            0x6A => Instr::Numeric(NumericInstr::I32Binary(IBinop::Add)),
            0x6B => Instr::Numeric(NumericInstr::I32Binary(IBinop::Sub)),
            0x6C => Instr::Numeric(NumericInstr::I32Binary(IBinop::Mul)),
            0x6D => Instr::Numeric(NumericInstr::I32Binary(IBinop::DivS)),
            0x6E => Instr::Numeric(NumericInstr::I32Binary(IBinop::DivU)),
            0x6F => Instr::Numeric(NumericInstr::I32Binary(IBinop::RemS)),
            0x70 => Instr::Numeric(NumericInstr::I32Binary(IBinop::RemU)),
            0x71 => Instr::Numeric(NumericInstr::I32Binary(IBinop::And)),
            0x72 => Instr::Numeric(NumericInstr::I32Binary(IBinop::Or)),
            0x73 => Instr::Numeric(NumericInstr::I32Binary(IBinop::Xor)),
            0x74 => Instr::Numeric(NumericInstr::I32Binary(IBinop::Shl)),
            0x75 => Instr::Numeric(NumericInstr::I32Binary(IBinop::ShrS)),
            0x76 => Instr::Numeric(NumericInstr::I32Binary(IBinop::ShrU)),
            0x77 => Instr::Numeric(NumericInstr::I32Binary(IBinop::Rotl)),
            0x78 => Instr::Numeric(NumericInstr::I32Binary(IBinop::Rotr)),

            0x79 => Instr::Numeric(NumericInstr::I64Unary(IUnop::Clz)),
            0x7A => Instr::Numeric(NumericInstr::I64Unary(IUnop::Ctz)),
            0x7B => Instr::Numeric(NumericInstr::I64Unary(IUnop::Popcnt)),
            0x7C => Instr::Numeric(NumericInstr::I64Binary(IBinop::Add)),
            0x7D => Instr::Numeric(NumericInstr::I64Binary(IBinop::Sub)),
            0x7E => Instr::Numeric(NumericInstr::I64Binary(IBinop::Mul)),
            0x7F => Instr::Numeric(NumericInstr::I64Binary(IBinop::DivS)),
            0x80 => Instr::Numeric(NumericInstr::I64Binary(IBinop::DivU)),
            0x81 => Instr::Numeric(NumericInstr::I64Binary(IBinop::RemS)),
            0x82 => Instr::Numeric(NumericInstr::I64Binary(IBinop::RemU)),
            0x83 => Instr::Numeric(NumericInstr::I64Binary(IBinop::And)),
            0x84 => Instr::Numeric(NumericInstr::I64Binary(IBinop::Or)),
            0x85 => Instr::Numeric(NumericInstr::I64Binary(IBinop::Xor)),
            0x86 => Instr::Numeric(NumericInstr::I64Binary(IBinop::Shl)),
            0x87 => Instr::Numeric(NumericInstr::I64Binary(IBinop::ShrS)),
            0x88 => Instr::Numeric(NumericInstr::I64Binary(IBinop::ShrU)),
            0x89 => Instr::Numeric(NumericInstr::I64Binary(IBinop::Rotl)),
            0x8A => Instr::Numeric(NumericInstr::I64Binary(IBinop::Rotr)),

            0x8B => Instr::Numeric(NumericInstr::F32Unary(FUnop::Abs)),
            0x8C => Instr::Numeric(NumericInstr::F32Unary(FUnop::Neg)),
            0x8D => Instr::Numeric(NumericInstr::F32Unary(FUnop::Ceil)),
            0x8E => Instr::Numeric(NumericInstr::F32Unary(FUnop::Floor)),
            0x8F => Instr::Numeric(NumericInstr::F32Unary(FUnop::Trunc)),
            0x90 => Instr::Numeric(NumericInstr::F32Unary(FUnop::Nearest)),
            0x91 => Instr::Numeric(NumericInstr::F32Unary(FUnop::Sqrt)),
            0x92 => Instr::Numeric(NumericInstr::F32Binary(FBinop::Add)),
            0x93 => Instr::Numeric(NumericInstr::F32Binary(FBinop::Sub)),
            0x94 => Instr::Numeric(NumericInstr::F32Binary(FBinop::Mul)),
            0x95 => Instr::Numeric(NumericInstr::F32Binary(FBinop::Div)),
            0x96 => Instr::Numeric(NumericInstr::F32Binary(FBinop::Min)),
            0x97 => Instr::Numeric(NumericInstr::F32Binary(FBinop::Max)),
            0x98 => Instr::Numeric(NumericInstr::F32Binary(FBinop::CopySign)),

            0x99 => Instr::Numeric(NumericInstr::F64Unary(FUnop::Abs)),
            0x9A => Instr::Numeric(NumericInstr::F64Unary(FUnop::Neg)),
            0x9B => Instr::Numeric(NumericInstr::F64Unary(FUnop::Ceil)),
            0x9C => Instr::Numeric(NumericInstr::F64Unary(FUnop::Floor)),
            0x9D => Instr::Numeric(NumericInstr::F64Unary(FUnop::Trunc)),
            0x9E => Instr::Numeric(NumericInstr::F64Unary(FUnop::Nearest)),
            0x9F => Instr::Numeric(NumericInstr::F64Unary(FUnop::Sqrt)),
            0xA0 => Instr::Numeric(NumericInstr::F64Binary(FBinop::Add)),
            0xA1 => Instr::Numeric(NumericInstr::F64Binary(FBinop::Sub)),
            0xA2 => Instr::Numeric(NumericInstr::F64Binary(FBinop::Mul)),
            0xA3 => Instr::Numeric(NumericInstr::F64Binary(FBinop::Div)),
            0xA4 => Instr::Numeric(NumericInstr::F64Binary(FBinop::Min)),
            0xA5 => Instr::Numeric(NumericInstr::F64Binary(FBinop::Max)),
            0xA6 => Instr::Numeric(NumericInstr::F64Binary(FBinop::CopySign)),

            0xA7 => Instr::Numeric(NumericInstr::I32WrapI64),
            0xA8 => Instr::Numeric(NumericInstr::I32TruncF32S),
            0xA9 => Instr::Numeric(NumericInstr::I32TruncF32U),
            0xAA => Instr::Numeric(NumericInstr::I32TruncF64S),
            0xAB => Instr::Numeric(NumericInstr::I32TruncF64U),
            0xAC => Instr::Numeric(NumericInstr::I64ExtendI32S),
            0xAD => Instr::Numeric(NumericInstr::I64ExtendI32U),
            0xAE => Instr::Numeric(NumericInstr::I64TruncF32S),
            0xAF => Instr::Numeric(NumericInstr::I64TruncF32U),
            0xB0 => Instr::Numeric(NumericInstr::I64TruncF64S),
            0xB1 => Instr::Numeric(NumericInstr::I64TruncF64U),
            0xB2 => Instr::Numeric(NumericInstr::F32ConvertI32S),
            0xB3 => Instr::Numeric(NumericInstr::F32ConvertI32U),
            0xB4 => Instr::Numeric(NumericInstr::F32ConvertI64S),
            0xB5 => Instr::Numeric(NumericInstr::F32ConvertI64U),
            0xB6 => Instr::Numeric(NumericInstr::F32DemoteF64),
            0xB7 => Instr::Numeric(NumericInstr::F64ConvertI32S),
            0xB8 => Instr::Numeric(NumericInstr::F64ConvertI32U),
            0xB9 => Instr::Numeric(NumericInstr::F64ConvertI64S),
            0xBA => Instr::Numeric(NumericInstr::F64ConvertI64U),
            0xBB => Instr::Numeric(NumericInstr::F64PromoteF32),
            0xBC => Instr::Numeric(NumericInstr::I32ReinterpretF32),
            0xBD => Instr::Numeric(NumericInstr::I64ReinterpretF64),
            0xBE => Instr::Numeric(NumericInstr::F32ReinterpretI32),
            0xBF => Instr::Numeric(NumericInstr::F64ReinterpretI64),

            0xC0 => Instr::Numeric(NumericInstr::I32Extend8S),
            0xC1 => Instr::Numeric(NumericInstr::I32Extend16S),
            0xC2 => Instr::Numeric(NumericInstr::I64Extend8S),
            0xC3 => Instr::Numeric(NumericInstr::I64Extend16S),
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
            0x7C..=0x7F | 0x6F | 0x70 => BlockType::Val(ValType::parse(data)?),
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
