// 1. take in src code -> tokens
// 2. goes thru its input and outputs next token recognized
// 3. does not need to buffer or save tokens
// 4. there will only be 1 method: next_token(), to go thru the source
//    code token by token, char by char.  outputs next token

// init lexer w/ source code & then repetedly call next_token()
// to reduce complexity we're using strings
// #![feature(split_inclusive)]

use crate::tokens::tokens::*;
use ascii::*;

// use std::convert::TryInto;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Vec<AsciiChar>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: vec![AsciiChar::Null],
        };

        l.read_char();
        l
    }
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = vec![AsciiChar::Null];
        } else {
            let ch = self.input.chars().nth(self.read_position).unwrap();
            self.ch = vec![AsciiChar::new(ch)];
        }

        self.position = self.read_position;
        self.read_position += 1;
        println!("self.ch = {:?}", self.ch);
    }

    fn read_identifier(&mut self) -> Vec<AsciiChar> {
        let mut literal: Vec<AsciiChar> = vec![];
        let mut c: AsciiChar = self.ch[0];

        while c.is_alphabetic() || c.is_ascii_digit() {
            literal.push(self.ch[0]);
            self.read_char();
            c = self.ch[0];
        }

        literal
    }

    fn match_token_type(&mut self) -> (TokenType, Vec<AsciiChar>) {
        let mut c = &self.ch;
        let mut default: bool = false;

        while c[0].is_ascii_whitespace() {
            self.read_char();
            c = &self.ch;
            println!("val of c at 73:  {:?}", &c)
        }
        println!("lexer.rs, line 82: value of c = {:?}", &c);

        let result = match c.as_slice() {
            [AsciiChar::Equal] => (TokenType::ASSIGN, vec![AsciiChar::Equal]),
            [AsciiChar::Semicolon] => (TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            [AsciiChar::ParenOpen] => (TokenType::LPAREN, vec![AsciiChar::ParenOpen]),
            [AsciiChar::ParenClose] => (TokenType::RPAREN, vec![AsciiChar::ParenClose]),
            [AsciiChar::CurlyBraceOpen] => (TokenType::LBRACE, vec![AsciiChar::CurlyBraceOpen]),
            [AsciiChar::CurlyBraceClose] => (TokenType::RBRACE, vec![AsciiChar::CurlyBraceClose]),
            [AsciiChar::Plus] => (TokenType::PLUS, vec![AsciiChar::Plus]),
            [AsciiChar::Comma] => (TokenType::COMMA, vec![AsciiChar::Comma]),
            [AsciiChar::Null] => (TokenType::EOF, vec![AsciiChar::Null]),
            _ => {
                if self.ch[0].is_ascii_alphanumeric() {
                    println!("lexer.rs, line 97, {:?}", c);
                    let literal = self.read_identifier();
                    default = true;
                    (Token::look_up_ident(literal.clone()), literal)
                } else {
                    println!("lexer.rs, line 105, {:?}", c);
                    default = true;
                    (TokenType::ILLEGAL, vec![AsciiChar::Null])
                }
            }
        };

        if !default {
            self.read_char();
        }

        result
    }

    pub fn next_token(&mut self) -> Token {
        let (token_type, literal) = self.match_token_type();
        // self.read_char();

        println!("line 108 self.ch {:?}", self.ch);
        Token::new(token_type, literal)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_char() {
        let input: String = String::from(";t; string");
        let mut l: Lexer = Lexer::new(input);

        assert_eq!(l.position, 0);
        assert_eq!(l.read_position, 1);
        assert_eq!(l.ch, vec![AsciiChar::Semicolon]);
        l.read_char();
        assert_eq!(l.position, 1);
        assert_eq!(l.read_position, 2);
        assert_eq!(l.ch, vec![AsciiChar::t]);
        l.read_char();
        assert_eq!(l.position, 2);
        assert_eq!(l.read_position, 3);
        assert_eq!(l.ch, vec![AsciiChar::Semicolon]);
    }

    #[test]
    fn read_identifier() {
        let mut l: Lexer = Lexer::new("let five cat".to_string());
        let literal = l.read_identifier();
        let ascii_chars = vec![AsciiChar::l, AsciiChar::e, AsciiChar::t];

        assert_eq!(literal[0], AsciiChar::l);
        assert_eq!(literal[1], AsciiChar::e);
        assert_eq!(literal[2], AsciiChar::t);
    }

    #[test]
    fn next_token() {
        let input = String::from("=+(){},;\nlet");

        let tests: [Token; 10] = [
            Token::new(TokenType::ASSIGN, vec![AsciiChar::Equal]),
            Token::new(TokenType::PLUS, vec![AsciiChar::Plus]),
            Token::new(TokenType::LPAREN, vec![AsciiChar::ParenOpen]),
            Token::new(TokenType::RPAREN, vec![AsciiChar::ParenClose]),
            Token::new(TokenType::LBRACE, vec![AsciiChar::CurlyBraceOpen]),
            Token::new(TokenType::RBRACE, vec![AsciiChar::CurlyBraceClose]),
            Token::new(TokenType::COMMA, vec![AsciiChar::Comma]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            Token::new(
                TokenType::LET,
                vec![AsciiChar::l, AsciiChar::e, AsciiChar::t],
            ),
            Token::new(TokenType::EOF, vec![AsciiChar::Null]),
        ];

        let mut l: Lexer = Lexer::new(input);

        for tt in tests.iter() {
            let tok = l.next_token();
            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }

    #[test]
    fn testing_advanced_input() {
        let input: String = String::from("let five = 5;\nlet ten = 10;\n\nlet add = fn(x, y) {\n\nx + y;\n\n};\n\nlet result = add(five, ten);\n");

        let tests: [Token; 36] = [
            Token::new(
                TokenType::LET,
                vec![AsciiChar::l, AsciiChar::e, AsciiChar::t],
            ),
            Token::new(
                TokenType::IDENT,
                vec![AsciiChar::f, AsciiChar::i, AsciiChar::v, AsciiChar::e],
            ),
            Token::new(TokenType::ASSIGN, vec![AsciiChar::Equal]),
            Token::new(TokenType::INT, vec![AsciiChar::_5]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            Token::new(
                TokenType::LET,
                vec![AsciiChar::l, AsciiChar::e, AsciiChar::t],
            ),
            Token::new(
                TokenType::IDENT,
                vec![AsciiChar::t, AsciiChar::e, AsciiChar::n],
            ),
            Token::new(TokenType::ASSIGN, vec![AsciiChar::Equal]),
            Token::new(TokenType::INT, vec![AsciiChar::_1, AsciiChar::_0]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            Token::new(
                TokenType::LET,
                vec![AsciiChar::l, AsciiChar::e, AsciiChar::t],
            ),
            Token::new(
                TokenType::IDENT,
                vec![AsciiChar::a, AsciiChar::d, AsciiChar::d],
            ),
            Token::new(TokenType::ASSIGN, vec![AsciiChar::Equal]),
            Token::new(TokenType::FUNCTION, vec![AsciiChar::f, AsciiChar::n]),
            Token::new(TokenType::LPAREN, vec![AsciiChar::ParenOpen]),
            Token::new(TokenType::IDENT, vec![AsciiChar::x]),
            Token::new(TokenType::COMMA, vec![AsciiChar::Comma]),
            Token::new(TokenType::IDENT, vec![AsciiChar::y]),
            Token::new(TokenType::RPAREN, vec![AsciiChar::ParenClose]),
            Token::new(TokenType::LBRACE, vec![AsciiChar::CurlyBraceOpen]),
            Token::new(TokenType::IDENT, vec![AsciiChar::x]),
            Token::new(TokenType::PLUS, vec![AsciiChar::Plus]),
            Token::new(TokenType::IDENT, vec![AsciiChar::y]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            Token::new(TokenType::RBRACE, vec![AsciiChar::CurlyBraceClose]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            Token::new(
                TokenType::LET,
                vec![AsciiChar::l, AsciiChar::e, AsciiChar::t],
            ),
            Token::new(
                TokenType::IDENT,
                vec![
                    AsciiChar::r,
                    AsciiChar::e,
                    AsciiChar::s,
                    AsciiChar::u,
                    AsciiChar::l,
                    AsciiChar::t,
                ],
            ),
            Token::new(TokenType::ASSIGN, vec![AsciiChar::Equal]),
            Token::new(
                TokenType::IDENT,
                vec![AsciiChar::a, AsciiChar::d, AsciiChar::d],
            ),
            Token::new(TokenType::LPAREN, vec![AsciiChar::ParenOpen]),
            Token::new(
                TokenType::IDENT,
                vec![AsciiChar::f, AsciiChar::i, AsciiChar::v, AsciiChar::e],
            ),
            Token::new(TokenType::COMMA, vec![AsciiChar::Comma]),
            Token::new(
                TokenType::IDENT,
                vec![AsciiChar::t, AsciiChar::e, AsciiChar::n],
            ),
            Token::new(TokenType::RPAREN, vec![AsciiChar::ParenClose]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
        ];

        let mut l: Lexer = Lexer::new(input);

        for tt in tests.iter() {
            let tok = l.next_token();
            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }
}
