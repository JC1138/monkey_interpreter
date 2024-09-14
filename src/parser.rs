use ast::Statement;

use crate::lexer::{Lexer, token::{Token, TokenType}};

mod arena_tree;
mod ast;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParseError(String);

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<ast::Statement>
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 0,
    EqualTo = 1, // ==
    GTLT = 2, // >, <
    Sum = 3, // +
    Mult = 4, // *,
    Prefix = 5, // -x, !x
    Call = 6, // x()
}

impl Precedence {
    fn get_precedence(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Eq | TokenType::NEq => Precedence::EqualTo,
            TokenType::LT | TokenType::GT => Precedence::GTLT,
            TokenType::Plus | TokenType::Dash => Precedence::Sum,
            TokenType::FSlash | TokenType::Star => Precedence::Mult,
            _ => Precedence::Lowest,
        }
    }
}

pub struct Parser {
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

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements: Vec<ast::Statement> = Vec::new();
        
        while self.cur_token.typ != TokenType::Eof {
            let statement = self.parse_statement()?;
            // println!("{statement:#?}                   ## parse_program");
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

        let expression = self.parse_expression(Precedence::Lowest)?;

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
        let expression = self.parse_expression(Precedence::Lowest)?;

        self.end_line();

        Ok(ast::Statement::Return {
                token: return_token,
                return_value: expression,
            }
        )
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let expression_token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        self.end_line();

        Ok(ast::Statement::ExpressionStatement {
            token: expression_token,
            expression: expression,
        })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<ast::Expression, ParseError> {
        let mut left = self.parse_prefix()?;

        while self.peek_token.typ != TokenType::Semicolon && precedence < Precedence::get_precedence(self.peek_token.typ) { // works with if ??
            match self.parse_infix(left.clone())? {
                Some(right) => left = right,
                None => return Ok(left),
            }
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<ast::Expression, ParseError> {
        // println!("Current token: {:?}", self.cur_token);
         match self.cur_token.typ {
            TokenType::Identifier => self.parse_identifier_expression(),
            TokenType::Int => self.parse_integer_expression(),
            TokenType::True | TokenType::False => self.parse_boolean_expression(),
            TokenType::Dash | TokenType::Exclam => self.parse_prefix_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            _ => Err(ParseError(format!("Unable to parse token in prefix position: {:?}", self.cur_token)))
        }
    }

    fn parse_infix(&mut self, left: ast::Expression) -> Result<Option<ast::Expression>, ParseError> {
        match self.peek_token.typ {
            TokenType::Eq | TokenType::NEq | TokenType::LT | TokenType::GT | TokenType::Plus | TokenType::Dash | TokenType::FSlash | TokenType::Star => {
                self.next_token();
                Ok(Some(self.parse_infix_expression(left)?))
            },
            _ => Ok(None),
        }
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

    fn parse_boolean_expression(&mut self) -> Result<ast::Expression, ParseError> {
        Ok(ast::Expression::Boolean { 
            token: self.cur_token.clone(), 
            value: match self.cur_token.literal.as_str() {
                "true" => true,
                "false" => false,
                _ => return Err(ParseError(format!("Unable to convert {} to bool!", self.cur_token.literal)))
            }
        })
    }

    fn parse_prefix_expression(&mut self) -> Result<ast::Expression, ParseError> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.to_string();
        self.next_token();
        let right = Box::new(self.parse_expression(Precedence::Prefix)?);
        Ok(ast::Expression::Prefix { 
            token, 
            operator, 
            right
        })
    }

    fn parse_grouped_expression(&mut self) -> Result<ast::Expression, ParseError> {
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest)?;

        self.next_token();
        
        if self.cur_token.typ != TokenType::RParen {
            return Err(ParseError(format!("Expected ')', got {:?}", self.cur_token)));
        }

        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<ast::Expression, ParseError> {
        let if_token = self.cur_token.clone();

        self.expect_next(TokenType::LParen)?;
        self.next_token();

        let condition = self.parse_expression(Precedence::Lowest)?;

        self.expect_next(TokenType::RParen)?;
        self.expect_next(TokenType::LBrace)?;
        
        let consequence = self.parse_block_statement()?;

        let mut alternative = None;
        if self.peek_token.typ == TokenType::Else {
            self.next_token();
            self.expect_next(TokenType::LBrace)?;
            alternative = Some(Box::new(self.parse_block_statement()?));
        }

        Ok(ast::Expression::If { 
            token: if_token, 
            condition: Box::new(condition), 
            consequence: Box::new(consequence), 
            alternative,
        })
    }

    fn parse_block_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let l_brace_token = self.cur_token.clone();
        let mut statements: Vec<Statement> = Vec::new();

        self.next_token();

        while self.cur_token.typ != TokenType::RBrace && self.cur_token.typ != TokenType::Eof {
            statements.push(self.parse_statement()?);
        }

        Ok(Statement::Block { 
            token: l_brace_token, 
            statements
         })
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> Result<ast::Expression, ParseError> {
        let operator_token = self.cur_token.clone();
        let precedence = Precedence::get_precedence(operator_token.typ);

        self.next_token();

        let right = self.parse_expression(precedence)?;

        Ok(ast::Expression::Infix { 
            operator: operator_token.literal.to_string(),
            token: operator_token,
            left: Box::new(left),
            right: Box::new(right)
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

    fn expect_next(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        if self.peek_token.typ != token_type {
            return Err(ParseError(format!("Expected {:?}, got: {:?}", token_type, self.peek_token)));
        }

        self.next_token();
        Ok(())
    }
}

mod tests {
    use ast::Statement;

    use super::*;

    #[allow(dead_code)]
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
            ast::Statement::construct_expression_statement(Token::new_identifier("foobar"), ast::Expression::construct_identifier_expression("foobar")),
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
            ast::Statement::construct_expression_statement(Token::new_exclam(), ast::Expression::construct_prefix_expression("!", ast::Expression::construct_integer_expression(5))),
            ast::Statement::construct_expression_statement(Token::new_dash(), ast::Expression::construct_prefix_expression("-", ast::Expression::construct_integer_expression(15))),
        ];

        do_test(program, expected);
    }

    #[test]
    fn test_infix_expression() {
        let program = r#"
            5 + 20;
            5 - 20;
            5 * 20;
            5 / 20;
            5 > 20;
            5 < 20;
            5 == 20;
            5 != 20;
        "#.to_string();

        let expected = vec![
            ast::Statement::construct_expression_statement(Token::new_int_i(5), ast::Expression::construct_infix_expression("+", ast::Expression::construct_integer_expression(5), ast::Expression::construct_integer_expression(20))),
            ast::Statement::construct_expression_statement(Token::new_int_i(5), ast::Expression::construct_infix_expression("-", ast::Expression::construct_integer_expression(5), ast::Expression::construct_integer_expression(20))),
            ast::Statement::construct_expression_statement(Token::new_int_i(5), ast::Expression::construct_infix_expression("*", ast::Expression::construct_integer_expression(5), ast::Expression::construct_integer_expression(20))),
            ast::Statement::construct_expression_statement(Token::new_int_i(5), ast::Expression::construct_infix_expression("/", ast::Expression::construct_integer_expression(5), ast::Expression::construct_integer_expression(20))),
            ast::Statement::construct_expression_statement(Token::new_int_i(5), ast::Expression::construct_infix_expression(">", ast::Expression::construct_integer_expression(5), ast::Expression::construct_integer_expression(20))),
            ast::Statement::construct_expression_statement(Token::new_int_i(5), ast::Expression::construct_infix_expression("<", ast::Expression::construct_integer_expression(5), ast::Expression::construct_integer_expression(20))),
            ast::Statement::construct_expression_statement(Token::new_int_i(5), ast::Expression::construct_infix_expression("==", ast::Expression::construct_integer_expression(5), ast::Expression::construct_integer_expression(20))),
            ast::Statement::construct_expression_statement(Token::new_int_i(5), ast::Expression::construct_infix_expression("!=", ast::Expression::construct_integer_expression(5), ast::Expression::construct_integer_expression(20))),
        ];

        do_test(program, expected);
    }

    #[test]
    fn test_integer_literal_expression() {
        let program = r#"
            5;
        "#.to_string();

        let expected = vec![
            ast::Statement::construct_expression_statement(Token::new_int("5"), ast::Expression::construct_integer_expression(5)),
        ];

        do_test(program, expected);
    }

    #[test]
    fn test_grouped_expression() {
        let program = r#"
            1 + (2 + 3) + 4;
            (5 + 5) * 2
            2 / (5 + 5);
            -(5 + 5);
            !(true == true)
        "#.to_string();

    let expected = vec![
        "((1 + (2 + 3)) + 4)",
        "((5 + 5) * 2)",
        "(2 / (5 + 5))",
        "(-(5 + 5))",
        "(!(true == true))",
    ];

    let l = Lexer::new(program);
    let mut parser = Parser::new(l);

    let parsed = parser.parse_program().unwrap();
    for p in &parsed.statements {
        println!("{}", p.dbg())
    }

    assert_eq!(parsed.statements.len(), expected.len(), "Expected {} statements, got {}", expected.len(), parsed.statements.len());
    for i in 0..expected.len() {
        assert_eq!(parsed.statements[i].dbg(), expected[i]);
    }
    }

    #[test]
    fn test_precidence() {
        let program = r#"
            a + add(b * c) + d;
            add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))
            add(a + b + c * d / f + g)
        "#.to_string();

        let expected = vec![
            "((a + add((b * c))) + d)",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            "add((((a + b) + ((c * d) / f)) + g))",
        ];

        let l = Lexer::new(program);
        let mut parser = Parser::new(l);

        let parsed = parser.parse_program().unwrap();

        assert_eq!(parsed.statements.len(), expected.len(), "Expected {} statements, got {}", expected.len(), parsed.statements.len());
        for i in 0..expected.len() {
            assert_eq!(parsed.statements[i].dbg(), expected[i]);
        }
    }

}
        // println!("Expression: {:#?}", expression);

        // // println!("Current token: {:?}", self.cur_token);
        
        // // println!("Current token after cycle: {:?}", self.cur_token);
        
        // println!("after eat_semicolon: {:?}", self.cur_token);
