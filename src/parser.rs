// mod crate::lexer;

//
// use lexer::Lexer;

use crate::lexer::Keyword;
use crate::lexer::Lexer;
use crate::lexer::Operation;
use crate::lexer::Token;
use crate::node;
use crate::node::Node;
use std::iter::Peekable;

pub struct Parser {
    lexer: Peekable<Lexer>,
    exprs: Vec<node::Expr>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
            exprs: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        self.parse_block();
    }

    fn parse_block(&mut self) {
        while let Some(token) = self.lexer.next() {
            println!("peek token in block : {:?}", token);

            if token == Token::Lparen {
                match self.parse_expr() {
                    Ok(expr) => self.exprs.push(expr),
                    Err(e) => println!("got error in parse_block {}", e),
                };
            } else {
                println!("it expect LParen in parse_block but got {:?}", token);
            }
        }
    }

    fn parse_expr(&mut self) -> Result<node::Expr, &'static str> {
        if let Some(token) = self.lexer.peek() {
            println!("token in parse expr {:?}", token);

            match token {
                Token::Operation(_) | Token::Keyword(_) => return self.parse_call(),
                _ => self.parse_list(),
            }
        } else {
            Err("no token in parse_expr")
        }
    }

    fn parse_call(&mut self) -> Result<node::Expr, &'static str> {
        println!("parse_call!");

        let func_name: node::FuncName;

        func_name = match self.lexer.next().unwrap() {
            Token::Operation(Operation::Plus) => node::FuncName::Plus,
            Token::Keyword(Keyword::Let) => node::FuncName::Let,
            Token::Keyword(Keyword::Print) => node::FuncName::Print,
            _ => return Err("func name invalid in parse_call"),
        };

        match self.parse_args() {
            Ok(args) => Ok(node::Expr::Call(node::Call {
                func_name: func_name,
                args: args,
            })),
            Err(e) => Err(e),
        }
    }

    fn parse_args(&mut self) -> Result<node::Args, &'static str> {
        println!("parse_args!");

        let mut args = node::Args { value: Vec::new() };

        while let Some(token) = self.lexer.peek() {
            if *token == Token::Rparen {
                self.lexer.next();
                break;
            }

            match self.parse_arg() {
                Ok(arg) => args.value.push(arg),
                Err(e) => return Err(e),
            }
        }

        Ok(args)
    }

    fn parse_arg(&mut self) -> Result<node::Arg, &'static str> {
        println!("parse_arg!");

        if let Some(token) = self.lexer.next() {
            println!("parse_arg, token is {:?}", token);

            match token {
                Token::Lparen => match self.parse_expr() {
                    Ok(expr) => Ok(node::Arg::Expr(expr)),
                    Err(e) => Err(e),
                },
                Token::Identifier(identifier) => Ok(node::Arg::Identifier(node::Identifier {
                    value: String::from(identifier),
                })),
                Token::Integer(number) => Ok(node::Arg::Number(node::Number { value: number })),
                Token::Literal(literal) => Ok(node::Arg::Literal(node::Literal { value: literal })),
                _ => Err("parse_arg did not match"),
            }
        } else {
            Err("did not match")
        }
    }

    fn parse_list(&mut self) -> Result<node::Expr, &'static str> {
        println!("inside parse list");

        let mut list = node::List { value: Vec::new() };

        while let Some(token) = self.lexer.peek() {
            if *token == Token::Rparen {
                self.lexer.next();
                break;
            }

            match self.parse_arg() {
                Ok(arg) => list.value.push(arg),
                Err(e) => return Err(e),
            }
        }

        Ok(node::Expr::List(list))
    }
}
