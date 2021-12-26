// Block           => Expr Expr | Expr
// Expr            => List | Call
// List            => ( Value )
// Call            => ( FuncName Value )
// FuncName        => + | print | let
// Value           => Value Value | Value

#[derive(Debug, Clone)]
pub struct Block {
    epxrs: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    List(List),
    Call(Call),
}

#[derive(Debug, Clone)]
pub struct Call {
    pub func_name: FuncName,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum FuncName {
    Plus,
    Print,
    Let,
}

#[derive(Debug, Clone)]
pub struct List {
    pub value: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Identifier(String),
    Number(usize),
    Literal(String),
    Expr(Expr),
}
