use std::fmt::Debug;
use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier {
        token: Token,
        value: String,
    },
    Integer {
        token: Token,
        value: isize,
    },
    Prefix {
        token: Token,
        operator: char,
        right: Box<Expression>,
    }
}

impl Expression {
    pub fn construct_identifier_expression(identifier: &str) -> Self {
        Expression::Identifier {
            token: Token::new_identifier(identifier),
            value: identifier.to_string()
        }
    }

    pub fn construct_integer_expression(value: isize) -> Self {
        Expression::Integer { 
            token: Token::new_int(&value.to_string()), 
            value
        }
    }

    pub fn construct_prefix_expression(operator: &str, right: Self) -> Self {
        Expression::Prefix { 
            token: match operator {
                "-" => Token::new_dash(),
                "!" => Token::new_exclam(),
                _ => panic!("{}", format!("Cannot use {operator} as a prefix!"))
            }, 
            operator: operator.chars().nth(0).unwrap(), 
            right: Box::new(right)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression {
        token: Token,
        expression: Expression,
    },
    Let {
        token: Token,
        name: Expression,
        value: Expression,
    },
    Return {
        token: Token,
        return_value: Expression,
    }
}

impl Statement {

    pub fn construct_expression_statement(first_token: Token, expression: Expression) -> Self {
        Self::Expression { token: first_token, expression }
    }

    pub fn construct_let_statement(identifier: String, value: isize) -> Self {
        Self::Let { 
            token: Token {
                typ: TokenType::Let, 
                literal: "let".to_string()
            }, 
            name: Expression::construct_identifier_expression(&identifier), 
            value: Expression::construct_integer_expression(value)
        }
    }

    pub fn construct_return_statement(return_value: Expression) -> Self {
        Self::Return { 
            token: Token {
                typ: TokenType::Return,
                literal: "return".to_string(),
            },
            return_value
        }
    }
}
