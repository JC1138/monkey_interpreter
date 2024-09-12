use std::fmt::Debug;

use crate::lexer::Token;

pub trait Expression: Debug {}

pub trait Statement: Debug {}

pub mod expressions {
    use super::*;
    
    #[derive(Debug, PartialEq)]
    pub struct Identifier {
        pub token: Token,
        pub value: String,
    }
    impl Expression for Identifier {}

    #[derive(Debug, PartialEq)]
    pub struct Integer {
        pub token: Token,
        pub value: usize,
    }
    impl Expression for Integer {}
}

pub mod statements {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct Expression<T: super::Expression> {
        pub token: Token,   // The first token in the expression
        pub expression: T,
    }
    impl<T: super::Expression> Statement for Expression<T> {}

    #[derive(Debug, PartialEq)]
    pub struct Let<T: super::Expression> {
        pub token: Token,
        pub name: expressions::Identifier,
        pub value: T,
    }
    impl<T: super::Expression> Statement for Let<T> {}

    #[derive(Debug, PartialEq)]
    pub struct Return<T: super::Expression> {
        pub token: Token,
        pub return_value: T,
    }
    impl<T: super::Expression> Statement for Return<T> {}
}
