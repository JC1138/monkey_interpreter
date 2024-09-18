use token::Token;
use helper::{is_digit, is_letter, is_str_char};

pub mod token;
mod helper;


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
            '+' => Token::new_plus(),
            ',' => Token::new_comma(),
            ';' => Token::new_semicolon(),
            '(' => Token::new_l_paren(),
            ')' => Token::new_r_paren(),
            '{' => Token::new_l_brace(),
            '}' => Token::new_r_brace(),
            '[' => Token::new_l_bracket(),
            ']' => Token::new_r_bracket(),
            '-' => Token::new_dash(),
            '/' => Token::new_f_slash(),
            '*' => Token::new_star(),
            '<' => Token::new_l_t(),
            '>' => Token::new_g_t(),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new_n_eq()
                }else {
                    Token::new_exclam()
                }
            },

            '"' => {
                self.read_char();
                Token::new_string(&self.read_string())
            }

            c if is_letter(c) => {
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

            c if is_digit(c) => {
                return Token::new_int(&self.read_int())
            },

            '\0' => Token::new_eof(),
            
            _ => Token::new_illegal(),
        };

        self.read_char();

        token
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
        let start = self.position;

        loop {
            self.read_char();
            if !matcher(self.ch) { break; }
        }

        self.src[start..self.position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        self.read_match(is_letter)
    }

    fn read_int(&mut self) -> String {
        self.read_match(is_digit)
    }

    fn read_string(&mut self) -> String {
        self.read_match(is_str_char)
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
        let src = "=+(){}[],;".to_string();

        let expected = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::LBracket, "["),
            (TokenType::RBracket, "]"),
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
            "foobar"
            "foo bar";
        "#.to_string();

        let expected = vec![
            Token::new_let(),
            Token::new_identifier("five"),
            Token::new_assign(),
            Token::new_int("5"),
            Token::new_semicolon(),
            Token::new_let(),
            Token::new_identifier("ten"),
            Token::new_assign(),
            Token::new_int("10"),
            Token::new_semicolon(),
            Token::new_let(),
            Token::new_identifier("add"),
            Token::new_assign(),
            Token::new_function(),
            Token::new_l_paren(),
            Token::new_identifier("x"),
            Token::new_comma(),
            Token::new_identifier("y"),
            Token::new_r_paren(),
            Token::new_l_brace(),
            Token::new_identifier("x"),
            Token::new_plus(),
            Token::new_identifier("y"),
            Token::new_semicolon(),
            Token::new_r_brace(),
            Token::new_semicolon(),
            Token::new_let(),
            Token::new_identifier("result"),
            Token::new_assign(),
            Token::new_identifier("add"),
            Token::new_l_paren(),
            Token::new_identifier("five"),
            Token::new_comma(),
            Token::new_identifier("ten"),
            Token::new_r_paren(),
            Token::new_semicolon(),
            Token::new_exclam(),
            Token::new_dash(),
            Token::new_f_slash(),
            Token::new_star(),
            Token::new_int("5"),
            Token::new_semicolon(),
            Token::new_int("5"),
            Token::new_l_t(),
            Token::new_int("10"),
            Token::new_g_t(),
            Token::new_int("5"),
            Token::new_semicolon(),
            // if statements:
            Token::new_if(),
            Token::new_l_paren(),
            Token::new_int("5"),
            Token::new_l_t(),
            Token::new_int("10"),
            Token::new_r_paren(),
            Token::new_l_brace(),
            Token::new_return(),
            Token::new_true(),
            Token::new_semicolon(),
            Token::new_r_brace(),
            Token::new_else(),
            Token::new_l_brace(),
            Token::new_return(),
            Token::new_false(),
            Token::new_semicolon(),
            Token::new_r_brace(),
            // 2-char tokens
            Token::new_int("10"),
            Token::new_eq(),
            Token::new_int("10"),
            Token::new_semicolon(),
            Token::new_int("10"),
            Token::new_n_eq(),
            Token::new_int("9"),
            Token::new_semicolon(),
            Token::new_eof(),
        ];


        let mut lexer = Lexer::new(src);

        for expected in expected {
            let token = lexer.next_token();
            assert_eq!(expected, token, "Expected {expected:?}, got {token:?}")
            // println!("{}: {token:?}", lexer.ch);
            // assert_eq!(expected.0, token.typ, "Expected type {:?}, got {:?}. Token: {:?}", expected.0, token.typ, token );
            // assert_eq!(expected.1, token.literal, "Expected literal {}, got {}. Token: {:?}", expected.1, token.literal, token);
        }

    }
}
