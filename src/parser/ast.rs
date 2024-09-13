use std::fmt::Debug;
use crate::lexer::Token;

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
