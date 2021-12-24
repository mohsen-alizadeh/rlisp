#![allow(dead_code)]
#![allow(unused_imports)]

mod lexer;
mod node;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let input = r#"
            (let name "Mohsen")

            (let marks (4 5 6))

            (print name)

            (print (+ marks))
        "#;

    let mut lexer = Lexer::new(input.to_string());

    let mut parser = Parser::new(lexer);

    let ast = parser.parse();

    println!("going to evaluate stuff");
}
