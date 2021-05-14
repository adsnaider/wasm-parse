use super::{expr::Expr, types::GlobalType};

#[derive(Debug)]
pub struct Global {
    t: GlobalType,
    init: Expr,
}
