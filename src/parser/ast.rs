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
        value: usize,
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

// pub trait Expression: Debug {}

// pub trait Statement: Debug {}

// pub mod expressions {
//     use super::*;
    
//     #[derive(Debug, PartialEq)]
//     pub struct Identifier {
//         pub token: Token,
//         pub value: String,
//     }
//     impl Expression for Identifier {}

//     #[derive(Debug, PartialEq)]
//     pub struct Integer {
//         pub token: Token,
//         pub value: usize,
//     }
//     impl Expression for Integer {}
// }

// pub mod statements {
//     use super::*;

//     #[derive(Debug)]
//     pub struct Expression {
//         pub token: Token,   // The first token in the expression
//         pub expression: Box<dyn super::Expression>,
//     }
//     impl Statement for Expression {}

//     #[derive(Debug)]
//     pub struct Let {
//         pub token: Token,
//         pub name: expressions::Identifier,
//         pub value: Box<dyn super::Expression>,
//     }
//     impl Statement for Let {}

//     #[derive(Debug)]
//     pub struct Return {
//         pub token: Token,
//         pub return_value: Box<dyn super::Expression>,
//     }
//     impl Statement for Return {}
// }
