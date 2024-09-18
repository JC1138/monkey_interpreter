#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,
    // identifiers + literals
    Identifier, // add, foobar, x, y, ...
    Int,        // 1343456
    String,
    // operators
    Assign,
    Plus,
    // delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Dash,
    FSlash,
    Star,
    LT,
    GT,
    Exclam,
    //compare
    Eq,
    NEq,
    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new_illegal() -> Self {
        Self { typ: TokenType::Illegal, literal: "illegal".to_string() }
    }
    pub fn new_eof() -> Self {
        Self { typ: TokenType::Eof, literal: "".to_string() }
    }
    // identifiers + literals
    pub fn new_identifier(identifier: &str) -> Self {
        Self { typ: TokenType::Identifier, literal: identifier.to_string() }
    }
    pub fn new_int(value: &str) -> Self {
        Self { typ: TokenType::Int, literal: value.to_string() }
    }
    pub fn new_int_i(value: isize) -> Self {
        Self::new_int(&value.to_string())
    }
    pub fn new_string(value: &str) -> Self {
        Self { typ: TokenType::String, literal: value.to_string() }
    }
    // operators
    pub fn new_assign() -> Self {
        Self { typ: TokenType::Assign, literal: "=".to_string() }
    }
    pub fn new_plus() -> Self {
        Self { typ: TokenType::Plus, literal: "+".to_string() }
    }
    // delimiters
    pub fn new_comma() -> Self {
        Self { typ: TokenType::Comma, literal: ",".to_string() }
    }
    pub fn new_semicolon() -> Self {
        Self { typ: TokenType::Semicolon, literal: ";".to_string() }
    }
    pub fn new_l_paren() -> Self {
        Self { typ: TokenType::LParen, literal: "(".to_string() }
    }
    pub fn new_r_paren() -> Self {
        Self { typ: TokenType::RParen, literal: ")".to_string() }
    }
    pub fn new_l_brace() -> Self {
        Self { typ: TokenType::LBrace, literal: "{".to_string() }
    }
    pub fn new_r_brace() -> Self {
        Self { typ: TokenType::RBrace, literal: "}".to_string() }
    }
    pub fn new_l_bracket() -> Self {
        Self { typ: TokenType::LBracket, literal: "[".to_string() }
    }
    pub fn new_r_bracket() -> Self {
        Self { typ: TokenType::RBracket, literal: "]".to_string() }
    }
    pub fn new_dash() -> Self {
        Self { typ: TokenType::Dash, literal: "-".to_string() }
    }
    pub fn new_f_slash() -> Self {
        Self { typ: TokenType::FSlash, literal: "/".to_string() }
    }
    pub fn new_star() -> Self {
        Self { typ: TokenType::Star, literal: "*".to_string() }
    }
    pub fn new_g_t() -> Self {
        Self { typ: TokenType::GT, literal: ">".to_string() }
    }
    pub fn new_l_t() -> Self {
        Self { typ: TokenType::LT, literal: "<".to_string() }
    }
    pub fn new_exclam() -> Self {
        Self { typ: TokenType::Exclam, literal: "!".to_string() }
    }
    //compare
    pub fn new_eq() -> Self {
        Self { typ: TokenType::Eq, literal: "==".to_string() }
    }
    pub fn new_n_eq() -> Self {
        Self { typ: TokenType::NEq, literal: "!=".to_string() }
    }
    // keywords
    pub fn new_function() -> Self {
        Self { typ: TokenType::Function, literal: "fn".to_string() }
    }
    pub fn new_let() -> Self {
        Self { typ: TokenType::Let, literal: "let".to_string() }
    }
    pub fn new_true() -> Self {
        Self { typ: TokenType::True, literal: "true".to_string() }
    }
    pub fn new_false() -> Self {
        Self { typ: TokenType::False, literal: "false".to_string() }
    }
    pub fn new_if() -> Self {
        Self { typ: TokenType::If, literal: "if".to_string() }
    }
    pub fn new_else() -> Self {
        Self { typ: TokenType::Else, literal: "else".to_string() }
    }
    pub fn new_return() -> Self {
        Self { typ: TokenType::Return, literal: "return".to_string() }
    }
}