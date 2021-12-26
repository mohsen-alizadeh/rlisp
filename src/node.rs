// Block           => Expr Expr | Expr
// Expr            => Call | Value
// Call            => ( FuncName Value )
// FuncName        => + | print | let
// Value           => Value Value | Value
// Value           => Identifier | Number | Literal | List

#[derive(Debug, Clone)]
pub struct Block {
    epxrs: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    List(Vec<Value>),
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
pub enum Value {
    Identifier(String),
    Number(usize),
    Literal(String),
    Expr(Expr),
    List(Vec<Value>),
}
