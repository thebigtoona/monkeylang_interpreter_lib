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
        let mut c: u8 = self.ch[0];

        while c.is_ascii_alphanumeric() {
            literal.push(self.ch[0]);
            self.read_char();
            c = self.ch[0];
        }

        (TokenType::ILLEGAL, literal)
    }

    fn read_number() {}

    fn match_token_type(&mut self, ch: u8) -> (TokenType, Vec<u8>) {
        match ch {
            b'=' => (TokenType::ASSIGN, vec![ch as u8]),
            b';' => (TokenType::SEMICOLON, vec![ch as u8]),
            b'(' => (TokenType::LPAREN, vec![ch as u8]),
            b')' => (TokenType::RPAREN, vec![ch as u8]),
            b'{' => (TokenType::LBRACE, vec![ch as u8]),
            b'}' => (TokenType::RBRACE, vec![ch as u8]),
            b'+' => (TokenType::PLUS, vec![ch as u8]),
            b',' => (TokenType::COMMA, vec![ch as u8]),
            0 => (TokenType::EOF, vec![0]),
            _ => {
                if ch.is_ascii_alphabetic() {
                    let (tok_type, literal) = self.read_identifier();
                    // (Token::look_up_ident(literal), literal)
                    (TokenType::IDENT, literal)
                } else if ch.is_ascii_digit() {
                    let (tok_type, literal) = self.read_identifier();
                    // digit here
                    // self.read_number
                    (TokenType::IDENT, literal)
                } else {
                    (TokenType::ILLEGAL, vec![ch as u8])
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        let (token_type, literal) = self.match_token_type(self.ch[0]);

        self.read_char();

        Token::new(token_type, literal)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_char() {
        let input: String = String::from("test string");
        let mut l: Lexer = Lexer::new(input);

        assert_eq!(l.position, 0);
        assert_eq!(l.read_position, 1);
        
        l.read_char();
        
        assert_eq!(l.position, 1);
        assert_eq!(l.read_position, 2);
    }

    #[test]
    fn read_identifier() {
        let mut l: Lexer = Lexer::new("let five cat".to_string());
        let tuple = l.read_identifier();
        let bytes = vec![b'l', b'e', b't'];

        assert_ne!(tuple.1.len(),(bytes.len()+1));
        assert_eq!(tuple.1.len(), bytes.len());
        
        assert_eq!(tuple.1[0], b'l');
        assert_eq!(tuple.1[1], b'e');
        assert_eq!(tuple.1[2], b't');
    }

    #[test]
    fn read_number() {
        // 
    }


    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");
        let tests: [Token; 9] = [
            Token::new(TokenType::ASSIGN, vec![b'=']),
            Token::new(TokenType::PLUS, vec![b'+']),
            Token::new(TokenType::LPAREN, vec![b'(']),
            Token::new(TokenType::RPAREN, vec![b')']),
            Token::new(TokenType::LBRACE, vec![b'{']),
            Token::new(TokenType::RBRACE, vec![b'}']),
            Token::new(TokenType::COMMA, vec![b',']),
            Token::new(TokenType::SEMICOLON, vec![b';']),
            Token::new(TokenType::EOF, vec![0]),
        ];

        let mut l: Lexer = Lexer::new(input);

        for tt in tests.iter() {
            let tok = l.next_token();
            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }
}
