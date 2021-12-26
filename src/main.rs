#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate lazy_static;

mod lexer;
mod node;
mod parser;
mod vm;

use lexer::Lexer;
use parser::Parser;

use vm::VM;

fn main() {
    let input = r#"
            (let name "Mohsen")

            (let marks (4 5 6))

            (print name)

            (print (+ marks))
        "#;

    let lexer = Lexer::new(input.to_string());

    let mut parser = Parser::new(lexer);

    parser.parse().unwrap();

    VM::new(parser.exprs).run();
}
