
#[derive(Debug)]
pub struct LexerError;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident, // add, foobar, x, y, ...
    Int, // 1343456

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Dash,
    FSlash,
    Star,
    LT,
    GT,
    Exclam,
    //two-character
    Eq,
    NEq,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

pub struct Lexer {
    src: String,
    chars: Vec<char>,
    position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        let chars: Vec<char> = src.chars().collect();
        let first_char = chars[0];
        Self {
            src,
            chars,
            position: 0,
            ch: first_char,
        }
    }

    pub fn next_token(&mut self) -> Token {

        self.eat_whitespace();
        let c = self.ch;

        let token = match c {
            '=' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token { typ: TokenType::Eq, literal: "==".to_string()}
                }else {
                    Self::get_single_char_token(TokenType::Assign, c)
                }
            },
            '+' => Self::get_single_char_token(TokenType::Plus, c),
            ',' => Self::get_single_char_token(TokenType::Comma, c),
            ';' => Self::get_single_char_token(TokenType::Semicolon, c),
            '(' => Self::get_single_char_token(TokenType::LParen, c),
            ')' => Self::get_single_char_token(TokenType::RParen, c),
            '{' => Self::get_single_char_token(TokenType::LBrace, c),
            '}' => Self::get_single_char_token(TokenType::RBrace, c),
            '-' => Self::get_single_char_token(TokenType::Dash, c),
            '/' => Self::get_single_char_token(TokenType::FSlash, c),
            '*' => Self::get_single_char_token(TokenType::Star, c),
            '<' => Self::get_single_char_token(TokenType::LT, c),
            '>' => Self::get_single_char_token(TokenType::GT, c),
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token { typ: TokenType::NEq, literal: "!=".to_string()}
                }else {
                    Self::get_single_char_token(TokenType::Exclam, c)
                }
            },

            c if Self::is_letter(c) => {
                let ident = self.read_identifier();

                return Token {
                    typ: match ident.as_str() {
                        "let" => TokenType::Let,
                        "fn" => TokenType::Function,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "return" => TokenType::Return,
                        _ => TokenType::Ident
                    },
                    literal: ident,
                };
            },

            c if Self::is_digit(c) => {
                return Token{ typ: TokenType::Int, literal: self.read_int() };
            },

            '\0' => Token { typ: TokenType::Eof, literal: "".to_string() },
            
            _ => Token { typ: TokenType::Illegal, literal: "".to_string() }
        };

        self.read_char();

        token
    }

    fn is_letter(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_digit(c: char) -> bool {
        matches!(c, '0'..='9')
    }

    fn read_char(&mut self) {
        self.ch = self.peek();
        self.position += 1;
    }

    fn peek(&self) -> char {
        let new_pos = self.position + 1;
        if new_pos >= self.chars.len() {
            '\0'
        }else {
            self.chars[new_pos]
        }
    }

    fn get_single_char_token(token_type: TokenType, c: char) -> Token {
        Token { typ: token_type, literal: c.to_string() }
    }

    fn read_match(&mut self, matcher: fn(char) -> bool) -> String {
        let position = self.position;

        loop {
            self.read_char();
            if !matcher(self.ch) { break; }
        }

        self.src[position..self.position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        self.read_match(Self::is_letter)
    }

    fn read_int(&mut self) -> String {
        self.read_match(Self::is_digit)
    }

    fn eat_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let src = "=+(){},;".to_string();

        let expected = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(src);

        for expected in expected {
            let token = lexer.next_token();

            assert_eq!(expected.0, token.typ);
            assert_eq!(expected.1, token.literal);
        }

    }

    #[test]
    fn complex_test() {
        let src = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10;
            10 != 9;
        "#.to_string();

        let expected = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Exclam, "!"),
            (TokenType::Dash, "-"),
            (TokenType::FSlash, "/"),
            (TokenType::Star, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::GT, ">"),
            (TokenType::Int, "5"),     
            (TokenType::Semicolon, ";"),

            // if statements:
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            // 2-char tokens
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(src);

        for expected in expected {
            let token = lexer.next_token();

            assert_eq!(expected.0, token.typ, "Expected type {:?}, got {:?}. Token: {:?}", expected.0, token.typ, token );
            assert_eq!(expected.1, token.literal, "Expected literal {}, got {}. Token: {:?}", expected.1, token.literal, token);
        }

    }
}
