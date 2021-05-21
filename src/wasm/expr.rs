use super::instr::Instr;

#[derive(Debug)]
pub struct Expr {
    pub instructions: Vec<Instr>,
}
