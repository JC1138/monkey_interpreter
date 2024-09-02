
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

    // Keywords
    Function,
    Let,
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

    pub fn next_token(&mut self) -> Result<Token, LexerError> {

        self.eat_whitespace();

        let token = match self.ch {
            '=' => Self::get_single_char_token(TokenType::Assign, self.ch),
            '+' => Self::get_single_char_token(TokenType::Plus, self.ch),
            ',' => Self::get_single_char_token(TokenType::Comma, self.ch),
            ';' => Self::get_single_char_token(TokenType::Semicolon, self.ch),
            '(' => Self::get_single_char_token(TokenType::LParen, self.ch),
            ')' => Self::get_single_char_token(TokenType::RParen, self.ch),
            '{' => Self::get_single_char_token(TokenType::LBrace, self.ch),
            '}' => Self::get_single_char_token(TokenType::RBrace, self.ch),
            c if Self::is_letter(c) => {
                let ident = self.read_identifier();
                let typ = match ident.as_str() {
                    "let" => TokenType::Let,
                    "fn" => TokenType::Function,
                    _ => TokenType::Ident
                };
    
                return Ok(Token { typ, literal: ident }) // Need to return early, since the loop ends with the position one char past the end of the identifier
            },

            c if Self::is_digit(c) => {
                return Ok(Token{ typ: TokenType::Int, literal: self.read_int() });
            }

            '\0' => Token { typ: TokenType::Eof, literal: "".to_string() },
            
            _ => Token { typ: TokenType::Illegal, literal: "".to_string() }
        };

        self.read_char();

        Ok(token)

        // Err(LexerError)
    }

    fn is_letter(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_digit(c: char) -> bool {
        matches!(c, '0'..='9')
    }

    fn read_char(&mut self) {
        self.position += 1;
        self.ch = if self.position >= self.src.len() {
            '\0'
        }else {
            self.chars[self.position]
        };
        // println!("The letter is: {}", self.ch);
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
            let token = lexer.next_token().unwrap();

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
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(src);

        for expected in expected {
            let token = lexer.next_token().unwrap();

            assert_eq!(expected.0, token.typ, "Expected type {:?}, got {:?}. Token: {:?}", expected.0, token.typ, token );
            assert_eq!(expected.1, token.literal, "Expected literal {}, got {}. Token: {:?}", expected.1, token.literal, token);
        }

    }
}
