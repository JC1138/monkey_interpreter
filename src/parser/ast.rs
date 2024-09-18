use std::fmt::Debug;
use crate::lexer::token::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier {
        token: Token,
        value: String,
    },
    Integer {
        token: Token,
        value: isize,
    },
    Boolean {
        token: Token,
        value: bool,
    },
    String {
        token: Token,
        value: String,
    },
    Array {
        token: Token, // '['
        elements: Vec<Self>
    },
    Prefix {
        token: Token,
        operator: String,
        right: Box<Self>,
    },
    Infix {
        token: Token, // The operator token
        left: Box<Self>,
        operator: String,
        right: Box<Self>
    },
    If {
        token: Token, // 'if',
        condition: Box<Self>,
        consequence: Box<Statement>, // Block statement
        alternative: Option<Box<Statement>>, // Block statement
    },
    Function {
        token: Token, // 'fn'
        params: Vec<Self>,
        body: Box<Statement> // Block statement
    },
    Call {
        token: Token, // '('
        function: Box<Self>, // Identifier or Function
        arguements: Vec<Self>,
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
            token: Token::new_int_i(value), 
            value
        }
    }

    pub fn construct_boolean_expression(value: bool) -> Self {
        Expression::Boolean { 
            token: if value {Token::new_true()} else {Token::new_false()}, 
            value,
        }
    }

    pub fn construct_string_expression(value: &str) -> Self {
        Expression::String { 
            token: Token::new_string(value), 
            value: value.to_string(),
        }
    }

    pub fn construct_prefix_expression(operator: &str, right: Self) -> Self {
        Expression::Prefix { 
            token: match operator {
                "-" => Token::new_dash(),
                "!" => Token::new_exclam(),
                _ => panic!("{}", format!("Cannot use {operator} as a prefix!"))
            }, 
            operator: operator.to_string(), 
            right: Box::new(right)
        }
    }

    pub fn construct_infix_expression(operator: &str, left: Self, right: Self) -> Self {
        Expression::Infix { 
            token: match operator {
                "+" => Token::new_plus(),
                "-" => Token::new_dash(),
                "*" => Token::new_star(),
                "/" => Token::new_f_slash(),
                ">" => Token::new_g_t(),
                "<" => Token::new_l_t(),
                "==" => Token::new_eq(),
                "!=" => Token::new_n_eq(),
                _ => panic!("{}", format!("Cannot use {operator} as a prefix!"))
            }, 
            left: Box::new(left), 
            operator: operator.to_string(), 
            right: Box::new(right),
        }
    }

    pub fn construct_if_expression(condition: Expression, consequence: Statement, alternative: Option<Statement>) -> Self {
        if let Statement::Block { .. } = &consequence {
            if let Some(alt) = &alternative {
                if let Statement::Block { .. } = &alt {
                } else {
                    panic!("Alternative must be a Block statement, got: {:?}", alternative);
                }
            }
        } else {
            panic!("Consequence must be a Block statement, got: {:?}", consequence);
        }

        Self::If { 
            token: Token::new_if(),
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: alternative.map(|alt| Box::new(alt)) // if let Some(alt) = alternative { Some(Box::new(alt)) } else { None }
        }
    }

    pub fn dbg(&self) -> String {
        match self {
            Self::Identifier { value, .. } => value.to_string(),
            Self::Integer { value, .. } => value.to_string(),
            Self::Boolean { value, .. } => value.to_string(),
            Self::String { value, .. } => value.to_string(),
            Self::Array { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|param| param.dbg())
                    .collect::<Vec<String>>()
                    .join(",");
                format!("[{}]", elements)
            }
            Self::Prefix { operator, right, .. } => format!("({}{})", operator, right.dbg()),
            Self::Infix { left, operator, right, .. } => format!("({} {} {})", left.dbg(), operator, right.dbg()),
            Self::If { token, condition, consequence, alternative } => {
                let mut out = format!("{} {} {}", token.literal, condition.dbg(), consequence.dbg());
                if let Some(alt) = alternative {
                    out += &format!("\nelse {}", alt.dbg());
                }

                out
            },
            Self::Function { token, params, body } => {
                let params = params
                                        .iter()
                                        .map(|param| param.dbg())
                                        .collect::<Vec<String>>()
                                        .join(",");
                format!("{}({}) {}", token.literal, params, body.dbg())
            },
            Self::Call { function, arguements, .. } => {
                let arguements = arguements
                                            .iter()
                                            .map(|param| param.dbg())
                                            .collect::<Vec<String>>()
                                            .join(",");
                format!("{}({})", function.dbg(), arguements)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    ExpressionStatement {
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
    },
    Block {
        token: Token, // '{'
        statements: Vec<Statement>
    }
}

impl Statement {
    pub fn construct_expression_statement(first_token: Token, expression: Expression) -> Self {
        Self::ExpressionStatement { token: first_token, expression }
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

    pub fn construct_block_statement(statements: Vec<Self>) -> Self {
        Self::Block { 
            token: Token::new_l_brace(), 
            statements 
        }
    }

    pub fn dbg(&self) -> String {
        match self {
            Self::Let { token, name, value } => format!("{} {} = {}", token.literal, name.dbg(), value.dbg()),
            Self::Return { token, return_value } => format!("{} {}", token.literal, return_value.dbg()),
            Self::ExpressionStatement { expression, .. } => expression.dbg(),
            Self::Block { statements, .. } => {
                let mut out = "{\n".to_string();
                for s in statements { out += &format!("\t{}\n", s.dbg()) }
                return out + " }"
            }
        }
    }
}
