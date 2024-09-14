use token::Token;

pub mod token;


#[derive(Debug)]
pub struct LexerError;

pub struct Lexer {
    src: String,
    chars: Vec<char>,
    position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        let chars: Vec<char> = src.chars().collect();
        let first_char = if chars.len() > 0 {
            chars[0]
        }else {
            '\0'
        };

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
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new_eq()
                }else {
                    Token::new_assign()
                }
            },
            '+' => Token::new_plus(), // Self::get_single_char_token(TokenType::Plus, c),
            ',' => Token::new_comma(), // Self::get_single_char_token(TokenType::Comma, c),
            ';' => Token::new_semicolon(), // Self::get_single_char_token(TokenType::Semicolon, c),
            '(' => Token::new_l_paren(), // Self::get_single_char_token(TokenType::LParen, c),
            ')' => Token::new_r_paren(), // Self::get_single_char_token(TokenType::RParen, c),
            '{' => Token::new_l_brace(), // Self::get_single_char_token(TokenType::LBrace, c),
            '}' => Token::new_r_brace(), // Self::get_single_char_token(TokenType::RBrace, c),
            '-' => Token::new_dash(), // Self::get_single_char_token(TokenType::Dash, c),
            '/' => Token::new_f_slash(), // Self::get_single_char_token(TokenType::FSlash, c),
            '*' => Token::new_star(), // Self::get_single_char_token(TokenType::Star, c),
            '<' => Token::new_l_t(), // Self::get_single_char_token(TokenType::LT, c),
            '>' => Token::new_g_t(), // Self::get_single_char_token(TokenType::GT, c),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new_n_eq()
                }else {
                    Token::new_exclam()
                }
            },

            c if Self::is_letter(c) => {
                return match self.read_identifier().as_str() {
                    "let" => Token::new_let(),
                    "fn" => Token::new_function(),
                    "if" => Token::new_if(),
                    "else" => Token::new_else(),
                    "true" => Token::new_true(),
                    "false" => Token::new_false(),
                    "return" => Token::new_return(),
                    i @ _ => Token::new_identifier(i)
                }
            },

            c if Self::is_digit(c) => {
                return Token::new_int(&self.read_int())
            },

            '\0' => Token::new_eof(),
            
            _ => Token::new_illegal(),
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
        self.ch = self.peek_char();
        self.position += 1;
    }

    fn peek_char(&self) -> char {
        let new_pos = self.position + 1;
        if new_pos >= self.chars.len() {
            '\0'
        }else {
            self.chars[new_pos]
        }
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
    use token::TokenType;

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
            (TokenType::Identifier, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Identifier, "x"),
            (TokenType::Comma, ","),
            (TokenType::Identifier, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Identifier, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Identifier, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "result"),
            (TokenType::Assign, "="),
            (TokenType::Identifier, "add"),
            (TokenType::LParen, "("),
            (TokenType::Identifier, "five"),
            (TokenType::Comma, ","),
            (TokenType::Identifier, "ten"),
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
            println!("{}: {token:?}", lexer.ch);
            assert_eq!(expected.0, token.typ, "Expected type {:?}, got {:?}. Token: {:?}", expected.0, token.typ, token );
            assert_eq!(expected.1, token.literal, "Expected literal {}, got {}. Token: {:?}", expected.1, token.literal, token);
        }

    }
}
