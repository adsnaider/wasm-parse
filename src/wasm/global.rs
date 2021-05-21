use super::{expr::Expr, types::GlobalType};

#[derive(Debug)]
pub struct Global {
    pub tpe: GlobalType,
    pub init: Expr,
}
