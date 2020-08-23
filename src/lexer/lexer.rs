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
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: ' ',
        };

        l.read_char();
        l
    }
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = ' ';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            '=' => Token {
                token_type: TokenType::ASSIGN,
                literal: self.ch,
            },
            ';' => Token {
                token_type: TokenType::SEMICOLON,
                literal: self.ch,
            },
            '(' => Token {
                token_type: TokenType::LPAREN,
                literal: self.ch,
            },
            ')' => Token {
                token_type: TokenType::RPAREN,
                literal: self.ch,
            },
            '{' => Token {
                token_type: TokenType::LBRACE,
                literal: self.ch,
            },
            '}' => Token {
                token_type: TokenType::RBRACE,
                literal: self.ch,
            },
            '+' => Token {
                token_type: TokenType::PLUS,
                literal: self.ch,
            },
            ',' => Token {
                token_type: TokenType::COMMA,
                literal: self.ch,
            },
            _ => Token {
                token_type: TokenType::EOF,
                literal: self.ch,
            },
        };

        self.read_char();
        tok
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        struct TestToken {
            expected_type: TokenType,
            expected_literal: char,
        }
        let tests: [TestToken; 9] = [
            // TestToken {
            //     expected_type: TokenType::LET,
            //     expected_literal: '',
            // },
            // TestToken {
            //     expected_type: TokenType::Type,
            //     expected_literal: '',
            // },
            // TestToken {
            //     expected_type: TokenType::Type,
            //     expected_literal: '',
            // },
            // TestToken {
            //     expected_type: TokenType::Type,
            //     expected_literal: '',
            // },
            // TestToken {
            //     expected_type: TokenType::Type,
            //     expected_literal: '',
            // },


            TestToken {
                expected_type: TokenType::ASSIGN,
                expected_literal: '=',
            },
            TestToken {
                expected_type: TokenType::PLUS,
                expected_literal: '+',
            },
            TestToken {
                expected_type: TokenType::LPAREN,
                expected_literal: '(',
            },
            TestToken {
                expected_type: TokenType::RPAREN,
                expected_literal: ')',
            },
            TestToken {
                expected_type: TokenType::LBRACE,
                expected_literal: '{',
            },
            TestToken {
                expected_type: TokenType::RBRACE,
                expected_literal: '}',
            },
            TestToken {
                expected_type: TokenType::COMMA,
                expected_literal: ',',
            },
            TestToken {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ';',
            },
            TestToken {
                expected_type: TokenType::EOF,
                expected_literal: ' ',
            },
        ];
        let mut l: Lexer = Lexer::new(input);

        for tt in tests.iter() {
            let tok = l.next_token();
            assert_eq!(tok.token_type, tt.expected_type);
            assert_eq!(tok.literal, tt.expected_literal);
        }
    }
}
