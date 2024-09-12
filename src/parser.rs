use ast::Statement;

use crate::lexer::{Lexer, Token, TokenType};

mod arena_tree;
mod ast;

#[derive(Debug)]
struct ParseError(String);

struct Program {
    pub statements: Vec<ast::Statement>
}

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

enum Precidence {
    Lowest = 0,
    EqualTo = 1, // ==
    GTLT = 2, // >, <
    Sum = 3, // +
    Mult = 4, // *,
    Prefix = 5, // -x, !x
    Call = 6, // x()
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
        let mut statements: Vec<ast::Statement> = Vec::new();
        
        while self.cur_token.typ != TokenType::Eof {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(Program {
            statements
        })
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, ParseError>  {
        match self.cur_token.typ {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(), //Err(ParseError(format!("Invalid token, expected start of statement, got: {:?}", self.cur_token.typ))),
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let let_token = self.cur_token.clone();

        if self.peek_token.typ != TokenType::Identifier {
            return Err(ParseError(format!("Invlaid `let` statement, expected Identifier, got: {:?}", self.peek_token.typ)));
        }

        self.next_token();

        let name = ast::Expression::Identifier {
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

        Ok(ast::Statement::Let {
                token: let_token,
                name,
                value: ast::Expression::Identifier {token: Token {typ: TokenType::Illegal, literal: "Illegal".to_string()}, value: "".to_string()},
            }
        )
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let return_token = self.cur_token.clone();

        loop {
            if self.cur_token.typ == TokenType::Semicolon {
                self.next_token();
                break;
            }
            self.next_token();
        }

        return Ok(ast::Statement::Return {
                token: return_token,
                return_value: ast::Expression::Identifier {token: Token {typ: TokenType::Illegal, literal: "Illegal".to_string()}, value: "".to_string()},
            }
        )
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, ParseError> {
        Err(ParseError("Error!".to_string()))
        // let expression_token = self.cur_token.clone();
        // // let expression = self.parse_expression()?;
        // Ok(Statement::Expression {
        //     token: expression_token,
        //     expression: expression,
        // })
    }

    // fn parse_expression(&mut self) -> Result<ast::Expression, ParseError> {

    // }
}

mod tests {
    use ast;

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

    fn test_let_statement(statement: &ast::Statement, expected_name: &str) {
        // let x: &dyn Any = statement;
        match statement {
            Statement::Let { token: _, name: _, value: _ } => {
                // match value {
                //     Expression::Identifier { token, value } => {

                //     },
                //     _ => 
                // }
                // assert_eq!(name.value, expected_name);
            },
            _ => panic!("Expected let statement with type Integer, got: {statement:#?}")
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
            match statement {
                Statement::Return { token, return_value: _ } => {
                    assert_eq!(token.typ, TokenType::Return);
                    // assert_eq!(return_value, 5);
                },
                _ => panic!("Expected return statement, got: {statement:#?}")
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let program = r#"
            "foobar;"
        "#.to_string();

        let l = Lexer::new(program);
        let mut parser = Parser::new(l);

        let parsed = parser.parse_program().unwrap();

        assert_eq!(parsed.statements.len(), 1, "Expected 1 statement, got {}", parsed.statements.len());

        match &parsed.statements[0] {
            Statement::Expression { token, expression } => {
                assert_eq!(token.typ, TokenType::Identifier);
                assert_eq!(token.literal, "foobar");

                match expression {
                    ast::Expression::Identifier { token, value } => {
                        assert_eq!(token.typ, TokenType::Identifier);
                        assert_eq!(token.literal, "foobar");
                        assert_eq!(value, "foobar");
                    },
                    _ => panic!("Expected identifier statement, got: {:#?}", expression)
                }
            },
            _ => panic!("Expected expression statement, got: {:#?}", parsed.statements[0])
        }
    }
}
