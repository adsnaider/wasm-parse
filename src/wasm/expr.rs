use super::instr::Instr;

#[derive(Debug)]
pub struct Expr {
    instructions: Vec<Instr>,
}
