// Block           => Expr Expr | Expr
// Expr            => Args | Call
// List            => ( Args )
// Call            => ( FuncName Args )
// FuncName        => + | print | let
// Args            => Arg Arg | Arg
// Arg             => Identifier | Number | Expr

#[derive(Debug)]
pub enum Node {
    Block(Block),
    Expr,
    Call,
    Number(Number),
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct Block {
    value: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    List(List),
    Call(Call),
}

#[derive(Debug)]
pub struct Call {
    pub func_name: FuncName,
    pub args: Args,
}

#[derive(Debug)]
pub enum FuncName {
    Plus,
    Print,
    Let,
}

#[derive(Debug)]
pub struct List {
    pub value: Vec<Arg>,
}

#[derive(Debug)]
pub struct Args {
    pub value: Vec<Arg>,
}

#[derive(Debug)]
pub enum Arg {
    Identifier(Identifier),
    Number(Number),
    Literal(Literal),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug)]
pub struct Literal {
    pub value: String,
}

#[derive(Debug)]
pub struct Number {
    pub value: usize,
}
