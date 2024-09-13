use crate::lexer::{Lexer, Token, TokenType};

mod arena_tree;
mod ast;

#[derive(Debug)]
struct ParseError(String);

struct Program {
    pub statements: Vec<ast::Statement>
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
        let mut statements: Vec<ast::Statement> = Vec::new();
        
        while self.cur_token.typ != TokenType::Eof {
            let statement = self.parse_statement()?;
            println!("{statement:#?}                   ## parse_program");
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
            _ => self.parse_expression_statement(),
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

        self.next_token();

        if self.cur_token.typ != TokenType::Assign {
            return Err(ParseError(format!("Invlaid `let` statement, expected Assign, got: {:?}", self.peek_token.typ)));
        }

        self.next_token();

        let expression = self.parse_expression(Precidence::Lowest)?;

        self.end_line();

        Ok(ast::Statement::Let {
                token: let_token,
                name,
                value: expression, //ast::Expression::Identifier {token: Token {typ: TokenType::Illegal, literal: "Illegal".to_string()}, value: "".to_string()}, // TODO: replace with actual expression
            }
        )
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let return_token = self.cur_token.clone();
        self.next_token();
        let expression = self.parse_expression(Precidence::Lowest)?;

        self.end_line();

        Ok(ast::Statement::Return {
                token: return_token,
                return_value: expression,
            }
        )
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let expression_token = self.cur_token.clone();
        let expression = self.parse_expression(Precidence::Lowest)?;

        self.end_line();

        Ok(ast::Statement::Expression {
            token: expression_token,
            expression: expression,
        })
    }

    fn parse_expression(&mut self, precidence: Precidence) -> Result<ast::Expression, ParseError> {
        self.parse_prefix()
    }

    fn parse_prefix(&mut self) -> Result<ast::Expression, ParseError> {
        println!("Current token: {:?}", self.cur_token);
         match self.cur_token.typ {
            TokenType::Identifier => self.parse_identifier_expression(),
            TokenType::Int => self.parse_integer_expression(),
            TokenType::Dash | TokenType::Exclam => {
                let token = self.cur_token.clone();
                let operator = self.cur_token.literal.chars().nth(0).expect(&format!("Token has no literal!: {:?}", self.cur_token));
                self.next_token();
                let right = Box::new(self.parse_expression(Precidence::Prefix)?);
                Ok(ast::Expression::Prefix { 
                    token, 
                    operator, 
                    right
                })
            }
            _ => Err(ParseError(format!("Unable to parse token in prefix position: {:#?}", self.cur_token)))
        }
    }

    fn parse_infix(&mut self) -> Result<ast::Expression, ParseError> {
        Err(ParseError("Not implemented!".to_string()))
    }

    fn parse_identifier_expression(&mut self) -> Result<ast::Expression, ParseError> {
        Ok(ast::Expression::Identifier { 
            token: self.cur_token.clone(), 
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_integer_expression(&mut self) -> Result<ast::Expression, ParseError> {
        Ok(ast::Expression::Integer { 
            token: self.cur_token.clone(), 
            value: match self.cur_token.literal.parse::<isize>() {
                Ok(val) => val,
                _ => return Err(ParseError(format!("Unable to convert {} to int!", self.cur_token.literal)))
            }
        })
    }

    fn end_line(&mut self) {
        self.next_token();
        self.eat_semicolon();
    }

    fn eat_semicolon(&mut self) {
        while self.cur_token.typ == TokenType::Semicolon {
            self.next_token();
        }
    }
}

mod tests {
    use ast::Statement;

    use super::*;

    fn do_test(program: String, expected: Vec<Statement>) {
        let l = Lexer::new(program);
        let mut parser = Parser::new(l);

        let parsed = parser.parse_program().unwrap();

        assert_eq!(parsed.statements.len(), expected.len(), "Expected {} statements, got {}", expected.len(), parsed.statements.len());
        for i in 0..expected.len() {
            assert_eq!(parsed.statements[i], expected[i]);
        }
    }

    #[test]
    fn basic_test() {
        let program = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#.to_string();

        let expected = vec![
            ast::Statement::construct_let_statement("x".to_string(), 5),
            ast::Statement::construct_let_statement("y".to_string(), 10),
            ast::Statement::construct_let_statement("foobar".to_string(), 838383),
        ];
        
        do_test(program, expected)
    }

    #[test]
    fn return_test() {
        let program = r#"
            return 5;
            return asdf;
            return 10;
        "#.to_string();

        let expected = vec![
            ast::Statement::construct_return_statement(ast::Expression::construct_integer_expression(5)),
            ast::Statement::construct_return_statement(ast::Expression::construct_identifier_expression("asdf")),
            ast::Statement::construct_return_statement(ast::Expression::construct_integer_expression(10)),
        ];

        do_test(program, expected);
    }

    #[test]
    fn test_identifier_expression() {
        let program = r#"
            foobar;
        "#.to_string();

        let expected = vec![
            ast::Statement::construct_expression_statement(Token {
                    typ: TokenType::Identifier,
                    literal: "foobar".to_string(),
                },
                ast::Expression::construct_identifier_expression("foobar"),
            ),
        ];

        do_test(program, expected);
    }

    #[test]
    fn test_prefix_expression() {
        let program = r#"
            !5;
            -15;
        "#.to_string();

        let expected = vec![
            ast::Statement::construct_expression_statement(Token {
                    typ: TokenType::Exclam,
                    literal: "!".to_string(),
                },
                ast::Expression::construct_prefix_expression("!", ast::Expression::construct_integer_expression(5))
            ),
            ast::Statement::construct_expression_statement(Token {
                    typ: TokenType::Dash,
                    literal: "-".to_string(),
                },
                ast::Expression::construct_prefix_expression("-", ast::Expression::construct_integer_expression(15))
            ),
        ];

        do_test(program, expected);
    }

    #[test]
    fn test_integer_literal_expression() {
        let program = r#"
            5;
        "#.to_string();

        let expected = vec![
            ast::Statement::construct_expression_statement(Token {
                    typ: TokenType::Int,
                    literal: "5".to_string(),
                }, 
                ast::Expression::construct_integer_expression(5),
            ),
        ];

        do_test(program, expected);
    }

}












        // println!("Expression: {:#?}", expression);

        // // println!("Current token: {:?}", self.cur_token);
        
        // // println!("Current token after cycle: {:?}", self.cur_token);
        
        // println!("after eat_semicolon: {:?}", self.cur_token);