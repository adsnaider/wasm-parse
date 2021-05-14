#[derive(Debug)]
pub struct ResultType {
    types: Vec<ValType>,
}

#[derive(Debug)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

#[derive(Debug)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(Debug)]
pub struct FuncType {
    params: ResultType,
    result: ResultType,
}

#[derive(Debug)]
pub struct GlobalType {
    mutability: Mutability,
    t: ValType,
}

#[derive(Debug)]
pub enum Mutability {
    Const,
    Mut,
}
