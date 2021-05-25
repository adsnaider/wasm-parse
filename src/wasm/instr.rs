#[derive(Debug)]
pub enum Instr {
    Undefined,
    Numeric(NumericInstr),
}

trait Instruction {
    fn execute();
}

#[derive(Debug)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Sign {
    Signed,
    Unsigned,
}

#[derive(Debug)]
pub enum NumericInstr {
    Const(ConstInstr),
    Unary(UnaryInstr),
    Binary(BinaryInstr),
    Test(TestInstr),
    Relation(RelationInstr),
    Extend(ExtendInstr),
    Wrap(WrapInstr),
    Trancate(TruncateInstr),
    TruncateSat(TruncateSatInstr),
    Demote(DemoteInstr),
    Promote(PromoteInstr),
    Convert(ConvertInstr),
    ReinterpetInstr(ReinterpetInstr),
}

#[derive(Debug)]
pub struct ConstInstr {
    tpe: Type,
}

#[derive(Debug)]
pub enum UnaryInstr {}

#[derive(Debug)]
pub enum IntUnaryInstr {}
#[derive(Debug)]
pub enum FloatUnaryInstr {}

#[derive(Debug)]
pub struct BinaryInstr {}
#[derive(Debug)]
pub struct TestInstr {}
#[derive(Debug)]
pub struct RelationInstr {}
#[derive(Debug)]
pub struct ExtendInstr {}
#[derive(Debug)]
pub struct WrapInstr {}
#[derive(Debug)]
pub struct TruncateInstr {}
#[derive(Debug)]
pub struct TruncateSatInstr {}
#[derive(Debug)]
pub struct DemoteInstr {}
#[derive(Debug)]
pub struct PromoteInstr {}
#[derive(Debug)]
pub struct ConvertInstr {}
#[derive(Debug)]
pub struct ReinterpetInstr {}
#[derive(Debug)]
pub enum IUnOp {
    Clz,
    Ctz,
    Popcnt,
}

#[derive(Debug)]
pub enum FUnOp {
    Clz,
    Ctz,
    Popcnt,
}

#[derive(Debug)]
pub struct Expr {
    pub instructions: Vec<Instr>,
}
