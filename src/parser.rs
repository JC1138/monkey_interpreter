use std::any::Any;

use crate::lexer::{Lexer, Token, TokenType};

mod arena_tree;
mod ast;

#[derive(Debug)]
struct ParseError(String);

struct Program {
    pub statements: Vec<Box<dyn ast::Statement>>
}

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Self {
            cur_token: lexer.next_token(),
            peek_token: lexer.next_token(),
            lexer,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements: Vec<Box<dyn ast::Statement>> = Vec::new();
        
        while self.cur_token.typ != TokenType::Eof {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(Program {
            statements
        })
    }

    fn parse_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError>  {
        match self.cur_token.typ {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => Err(ParseError(format!("Invalid token, expected start of statement, got: {:?}", self.cur_token.typ))),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let let_token = self.cur_token.clone();

        if self.peek_token.typ != TokenType::Identifier {
            return Err(ParseError(format!("Invlaid `let` statement, expected Identifier, got: {:?}", self.peek_token.typ)));
        }

        self.next_token();

        let name = ast::expressions::Identifier {
            value: self.cur_token.literal.to_string(),
            token: self.cur_token.clone(),
        };

        if self.peek_token.typ != TokenType::Assign {
            return Err(ParseError(format!("Invlaid `let` statement, expected Assign, got: {:?}", self.peek_token.typ)));
        }

        self.next_token();

        loop {
            if self.cur_token.typ == TokenType::Semicolon {
                self.next_token();
                break;
            }
            self.next_token();
        }

        Ok(Box::new(
            ast::statements::Let {
                token: let_token,
                name,
                value: ast::expressions::Identifier {token: Token {typ: TokenType::Illegal, literal: "Illegal".to_string()}, value: "".to_string()},
            }
        ))
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn ast::Statement>, ParseError> {
        let return_token = self.cur_token.clone();

        loop {
            if self.cur_token.typ == TokenType::Semicolon {
                self.next_token();
                break;
            }
            self.next_token();
        }

        return Ok(Box::new(
            ast::statements::Return {
                token: return_token,
                return_value: ast::expressions::Identifier {token: Token {typ: TokenType::Illegal, literal: "Illegal".to_string()}, value: "".to_string()},
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use ast::{expressions, statements};

    use super::*;

    #[test]
    fn basic_test() {
        let program = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#.to_string();

        let l = Lexer::new(program);
        let mut parser = Parser::new(l);

        let parsed = parser.parse_program().unwrap();

        let expected_identifiers = [
            "x".to_owned(),
            "y".to_owned(),
            "foobar".to_owned()
        ];

        assert_eq!(parsed.statements.len(), expected_identifiers.len(), "Expected {} statements, got {}", expected_identifiers.len(), parsed.statements.len());

        for i in 0..expected_identifiers.len() {
            test_let_statement(&parsed.statements[i], &expected_identifiers[i]);
        }
    }

    fn test_let_statement(statement: &Box<dyn ast::Statement>, expected_name: &str) {
        let x: &dyn Any = statement;
        if let Some(let_statement) = x.downcast_ref::<statements::Let<expressions::Integer>>() {
            assert_eq!(let_statement.name.value, expected_name);
        } else {
            panic!("Expected let statement with type Integer, got: {statement:#?}")
        }
    }

    #[test]
    fn return_test() {
        let program = r#"
            return 5;
            return asdf;
            return 10;
        "#.to_string();

        let l = Lexer::new(program);
        let mut parser = Parser::new(l);

        let parsed = parser.parse_program().unwrap();

        assert_eq!(parsed.statements.len(), 3, "Expected 3 statements, got {}", parsed.statements.len());

        for statement in parsed.statements {
            let x: &dyn Any = &statement;
            if let Some(return_statement) = x.downcast_ref::<statements::Return<expressions::Integer>>() {
                assert_eq!(return_statement.token.typ, TokenType::Return);
                assert_eq!(return_statement.return_value.value, 5);
            } else {
                panic!("Expected return statement, got: {x:#?}")
            }
        }
    }
}
