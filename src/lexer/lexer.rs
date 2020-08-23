// 1. take in src code -> tokens
// 2. goes thru its input and outputs next token recognized
// 3. does not need to buffer or save tokens
// 4. there will only be 1 method: next_token(), to go thru the source
//    code token by token, char by char.  outputs next token

// init lexer w/ source code & then repetedly call next_token()
// to reduce complexity we're using strings

use crate::tokens::tokens::*;
// use std::convert::TryInto;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: vec![0],
        };

        l.read_char();
        l
    }
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = vec![0];
        } else {
            let ch = self.input.chars().nth(self.read_position).unwrap() as u8;
            self.ch = vec![ch];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> (TokenType, Vec<u8>) {
        let mut literal: Vec<u8> = vec![];
        let mut c: char = self.ch[0] as char;

        // let start = self.position;
        // let mut ahead = self.read_position;

        while c.is_alphabetic() {
            literal.push(self.ch[0]);
            
            self.read_char();
            c = self.ch[0] as char;
            // ahead = self.read_position;
        }

        (TokenType::ILLEGAL, literal)
    }

    fn match_token_type(&mut self, ch: char) -> (TokenType, Vec<u8>) {
        match ch {
            '=' => (TokenType::ASSIGN, vec![ch as u8]),
            ';' => (TokenType::SEMICOLON, vec![ch as u8]),
            '(' => (TokenType::LPAREN, vec![ch as u8]),
            ')' => (TokenType::RPAREN, vec![ch as u8]),
            '{' => (TokenType::LBRACE, vec![ch as u8]),
            '}' => (TokenType::RBRACE, vec![ch as u8]),
            '+' => (TokenType::PLUS, vec![ch as u8]),
            ',' => (TokenType::COMMA, vec![ch as u8]),
            _ => {
                if ch.is_alphabetic() {
                    let (tok_type, literal) = self.read_identifier();
                    // (Token::look_up_ident(literal), literal)
                    (TokenType::IDENT, literal)
                } else {
                    (TokenType::ILLEGAL, vec![ch as u8])
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        let (token_type, literal) = self.match_token_type(self.ch[0] as char);

        self.read_char();

        Token {
            token_type,
            literal
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let tests: [Token; 9] = [
            Token {
                token_type: TokenType::ASSIGN,
                literal: vec![b'='],
            },
            Token {
                token_type: TokenType::PLUS,
                literal: vec![b'+'],
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: vec![b'('],
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: vec![b')'],
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: vec![b'{'],
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: vec![b'}'],
            },
            Token {
                token_type: TokenType::COMMA,
                literal: vec![b','],
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: vec![b';'],
            },
            Token {
                token_type: TokenType::ILLEGAL,
                literal: vec![0],
            },
        ];

        let mut l: Lexer = Lexer::new(input);

        for tt in tests.iter() {
            let tok = l.next_token();
            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }
}
