// mod crate::lexer;

//
// use lexer::Lexer;

use crate::lexer::Keyword;
use crate::lexer::Lexer;
use crate::lexer::Operation;
use crate::lexer::Token;
use crate::node;
use std::iter::Peekable;

pub struct Parser {
    lexer: Peekable<Lexer>,
    pub exprs: Vec<node::Expr>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
            exprs: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), &'static str> {
        self.parse_block()?;

        Ok(())
    }

    fn parse_block(&mut self) -> Result<(), &'static str> {
        while let Some(token) = self.lexer.next() {
            if token == Token::Lparen {
                let expr = self.parse_expr()?;
                self.exprs.push(expr);
            } else {
                return Err("it expect LParen in parse_block");
            }
        }

        Ok(())
    }

    fn parse_expr(&mut self) -> Result<node::Expr, &'static str> {
        if let Some(token) = self.lexer.peek() {
            match token {
                Token::Operation(_) | Token::Keyword(_) => return self.parse_call(),
                _ => self.parse_list(),
            }
        } else {
            Err("no token in parse_expr")
        }
    }

    fn parse_call(&mut self) -> Result<node::Expr, &'static str> {
        let func_name: node::FuncName;

        func_name = match self.lexer.next().unwrap() {
            Token::Operation(Operation::Plus) => node::FuncName::Plus,
            Token::Keyword(Keyword::Let) => node::FuncName::Let,
            Token::Keyword(Keyword::Print) => node::FuncName::Print,
            _ => return Err("func name invalid in parse_call"),
        };

        Ok(node::Expr::Call(node::Call {
            func_name: func_name,
            args: self.parse_args()?,
        }))
    }

    fn parse_args(&mut self) -> Result<Vec<node::Value>, &'static str> {
        let mut args: Vec<node::Value> = Vec::new();

        while let Some(token) = self.lexer.peek() {
            if *token == Token::Rparen {
                self.lexer.next();
                break;
            }

            args.push(self.parse_arg()?);
        }

        Ok(args)
    }

    fn parse_arg(&mut self) -> Result<node::Value, &'static str> {
        if let Some(token) = self.lexer.next() {
            match token {
                Token::Lparen => Ok(node::Value::Expr(self.parse_expr()?)),
                Token::Identifier(identifier) => {
                    Ok(node::Value::Identifier(String::from(identifier)))
                }
                Token::Integer(number) => Ok(node::Value::Number(number)),
                Token::Literal(literal) => Ok(node::Value::Literal(literal)),
                _ => Err("parse_arg did not match"),
            }
        } else {
            Err("did not match")
        }
    }

    fn parse_list(&mut self) -> Result<node::Expr, &'static str> {
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
