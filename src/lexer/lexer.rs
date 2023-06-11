
use std::fmt::Display;

use anyhow::{Ok, Result};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Int(String),

    Illegal,
    EOF,

    Assign,

    Bang,
    Dash,
    ForwardSlash,
    Asterisk,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    If,
    Else,
    For,
    Return,
    False,
    True,

    Function,
    Let,

    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "Indent: {}", x),
            Token::Int(x) => write!(f, "Int: {}", x),
            Token::Illegal => write!(f, "Illegal"),
            Token::EOF => write!(f, "EOF"),
            Token::Assign => write!(f, "Assign"),
            Token::Bang => write!(f, "Bang"),
            Token::Dash => write!(f, "Dash"),
            Token::ForwardSlash => write!(f, "ForwardSlash"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::For => write!(f, "For"),
            Token::Return => write!(f, "Return"),
            Token::False => write!(f, "False"),
            Token::True => write!(f, "True"),
            Token::Function => write!(f, "Function"),
            Token::Let => write!(f, "Let"),
            Token::Plus => write!(f, "Plus"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Lparen => write!(f, "Lparen"),
            Token::Rparen => write!(f, "Rparen"),
            Token::LSquirly => write!(f, "LSquirly"),
            Token::RSquirly => write!(f, "RSquirly"),
        }
    }
}

pub struct Lexer {
    position: usize,
    read_pasition: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new() -> Lexer {
        let lex = Lexer {
            position: 0,
            read_pasition: 0,
            ch: 0,
            input: "".into(),
        };

        return lex;
    }

    pub fn parse(&mut self, input: String) -> Result<Vec<Token>> {
        self.input = input.into();

        let mut tokens: Vec<Token> = Vec::new();
        let mut curr: Token = self.next_token()?;
        let _= curr;
        while !(self.position >= self.input.len()) {
            curr = self.next_token()?;
            tokens.push(curr)
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let tk = match self.ch {
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'+' => Token::Plus,
            b';' => Token::Semicolon,
            b',' => Token::Comma,
            b'-' => Token::Dash,
            b'/' => Token::ForwardSlash,
            b'*' => Token::Asterisk,
            b'>' => Token::GreaterThan,
            b'<' => Token::LessThan,
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_indent();

                return Ok(match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "for" => Token::For,
                    "true" => Token::True,
                    "false" => Token::False,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                });
            }

            b'0'..=b'9' => return Ok(Token::Int(self.read_int())),

            0 => Token::EOF,
            _ => Token::Illegal,
        };

        self.read_char();
        Ok(tk)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_pasition >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_pasition]
        }

        self.position = self.read_pasition;
        self.read_pasition += 1;
    }

    fn peek(&mut self) -> u8 {
        if self.read_pasition >= self.input.len() {
            return 0;
        } else {
            self.input[self.read_pasition]
        }
    }

    fn read_indent(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char()
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn get_next_token() -> Result<()> {
        let input = "=+(){},;";
        let mut lexer = Lexer::new();
        

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::LSquirly,
            Token::RSquirly,
            Token::Comma,
            Token::Semicolon,
        ];

            assert_eq!(tokens, lexer.parse(input.into())?);

        return Ok(());
    }

    #[test]
    fn get_next_complete() -> Result<()> {
        let input = r#"let five = 5;
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
        "#;

        let mut lex = Lexer::new();

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::LSquirly,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RSquirly,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Dash,
            Token::ForwardSlash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::GreaterThan,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::Rparen,
            Token::LSquirly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RSquirly,
            Token::Else,
            Token::LSquirly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RSquirly,
            Token::Int(String::from("10")),
            Token::Equal,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("10")),
            Token::NotEqual,
            Token::Int(String::from("9")),
            Token::Semicolon,
            Token::EOF,
        ];

            assert_eq!(tokens,  lex.parse(input.into())?);

        return Ok(());
    }

    #[test]
    fn test_parse() -> Result<()>{
        let input = "1+2/3*9".into();
        let tokens = vec![
            Token::Int("1".into()),
            Token::Plus,
            Token::Int("2".into()),
            Token::ForwardSlash,
            Token::Int("3".into()),
            Token::Asterisk,
            Token::Int("9".into()),
        ];

        let mut lexer = Lexer::new();

        let res = lexer.parse(input)?;
        
        assert_eq!(tokens, res);

        Ok(())
    }
}
