use std::{any::Any, fmt::Debug};

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression;

pub trait Statement: Debug {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression
}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl Statement for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
