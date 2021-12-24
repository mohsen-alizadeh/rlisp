pub struct Lexer {
    position: usize,
    input: Vec<char>,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn parse_literal(&mut self) -> Option<Token> {
        let mut literal = String::new();
        self.position += 1;

        loop {
            if let Some(chr) = self.input.get(self.position) {
                self.position += 1;

                if *chr == '"' {
                    break;
                } else {
                    literal.push(*chr);
                }
            } else {
                break;
            }
        }

        Some(Token::Literal(literal))
    }

    fn parse_identifier(&mut self) -> Option<Token> {
        let mut identifier = String::new();

        loop {
            if let Some(chr) = self.input.get(self.position) {
                match chr {
                    'a'..='z' => {
                        identifier.push(*chr);
                        self.position += 1;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "let" => Some(Token::Keyword(Keyword::Let)),
            "print" => Some(Token::Keyword(Keyword::Print)),
            _ => Some(Token::Identifier(identifier)),
        }
    }

    fn parse_uint(&mut self) -> Option<Token> {
        let mut number = String::new();

        loop {
            if let Some(chr) = self.input.get(self.position) {
                match chr {
                    '0'..='9' => {
                        number.push(*chr);
                        self.position += 1;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        Some(Token::Integer(number.parse::<usize>().unwrap()))
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(chr) = self.input.get(self.position) {
                match chr {
                    '0'..='9' => return self.parse_uint(),
                    ' ' | '\n' => self.position += 1,
                    '(' => {
                        self.position += 1;
                        return Some(Token::Lparen);
                    }
                    ')' => {
                        self.position += 1;
                        return Some(Token::Rparen);
                    }
                    'a'..='z' => return self.parse_identifier(),
                    '"' => return self.parse_literal(),
                    '+' => {
                        self.position += 1;
                        return Some(Token::Operation(Operation::Plus));
                    }

                    _ => return None,
                }
            } else {
                return None;
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Keyword {
    Let,
    Print,
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Plus,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Integer(usize),
    Identifier(String),
    Keyword(Keyword),
    Literal(String),
    Lparen,
    Rparen,
    Operation(Operation),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = r#"
            (let name "Mohsen")

            (let marks (4 5 6))

            (print name)

            (print (+ marks))
        "#;

        let mut lexer = Lexer::new(input.to_string());

        // (let name "Mohsen")
        assert_eq!(lexer.next().unwrap(), Token::Lparen);
        assert_eq!(lexer.next().unwrap(), Token::Keyword(Keyword::Let));
        assert_eq!(lexer.next().unwrap(), Token::Identifier("name".to_string()));
        assert_eq!(lexer.next().unwrap(), Token::Literal("Mohsen".to_string()));
        assert_eq!(lexer.next().unwrap(), Token::Rparen);

        // (let marks (4 5 6))
        assert_eq!(lexer.next().unwrap(), Token::Lparen);
        assert_eq!(lexer.next().unwrap(), Token::Keyword(Keyword::Let));
        assert_eq!(
            lexer.next().unwrap(),
            Token::Identifier("marks".to_string())
        );
        assert_eq!(lexer.next().unwrap(), Token::Lparen);
        assert_eq!(lexer.next().unwrap(), Token::Integer(4));
        assert_eq!(lexer.next().unwrap(), Token::Integer(5));
        assert_eq!(lexer.next().unwrap(), Token::Integer(6));
        assert_eq!(lexer.next().unwrap(), Token::Rparen);
        assert_eq!(lexer.next().unwrap(), Token::Rparen);

        // (print name)
        assert_eq!(lexer.next().unwrap(), Token::Lparen);
        assert_eq!(lexer.next().unwrap(), Token::Keyword(Keyword::Print));
        assert_eq!(lexer.next().unwrap(), Token::Identifier("name".to_string()));
        assert_eq!(lexer.next().unwrap(), Token::Rparen);

        // (print (+ marks))
        assert_eq!(lexer.next().unwrap(), Token::Lparen);
        assert_eq!(lexer.next().unwrap(), Token::Keyword(Keyword::Print));
        assert_eq!(lexer.next().unwrap(), Token::Lparen);
        assert_eq!(lexer.next().unwrap(), Token::Operation(Operation::Plus));
        assert_eq!(
            lexer.next().unwrap(),
            Token::Identifier("marks".to_string())
        );
        assert_eq!(lexer.next().unwrap(), Token::Rparen);
        assert_eq!(lexer.next().unwrap(), Token::Rparen);
    }
}
