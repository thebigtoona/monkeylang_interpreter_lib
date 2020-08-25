use crate::tokens::tokens::*;
use ascii::*;

/// Represents a Lexer for monkey lang
///
/// # Parameters
///
/// * `input` - value to lex
///
/// * `position` - value that represents the current position of the
/// lexer
///
/// * `read_position` - value that represents the position the lexer is
/// currently reading (typically one ahead of the current position value)
///
/// * `ch` - vector that represents the characters the lexer is
/// currently matching / working with
///
/// # Remarks
///
/// * instanciate this with a mutable variable. the lexer needs to be mutable to
/// adjust the positions and ch values.
///
struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Vec<AsciiChar>,
}

impl Lexer {
    /// returns a new Lexer construct
    ///
    /// # Arguments
    ///
    /// * `input` - a `String` value to lex
    ///
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
    /// returns next position character in Lexer input as an AsciiChar, without
    /// modifying the Lexer or advancing the position/read_position parameters
    ///
    /// # Arguments
    ///
    /// * `&self` - a reference to the Lexer construct being used
    ///
    fn peek_char(&self) -> AsciiChar {
        if self.read_position >= self.input.len() {
            AsciiChar::Null
        } else {
            let ch = self.input.chars().nth(self.read_position).unwrap();
            AsciiChar::new(ch)
        }
    }
    /// advances `position` and `next_position` on the lexer
    ///
    /// # Arguments
    ///
    /// * `&mut self` - a mutable reference to the Lexer construct being used
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = vec![AsciiChar::Null];
        } else {
            let ch = self.input.chars().nth(self.read_position).unwrap();
            self.ch = vec![AsciiChar::new(ch)];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// returns a vector of AsciiChar items that represent the entire literal
    /// the lexer is attempting to match/parse against. used to read IDENT and
    /// keyword vals from the input String.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - a mutable reference to the Lexer construct being used
    ///
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

    /// returns `(TokenType Vec<AsciiChar>)` that references a match from the
    /// lexer.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - a mutable reference to the Lexer construct being used
    ///
    fn match_token_type(&mut self) -> (TokenType, Vec<AsciiChar>) {
        let mut default: bool = false;

        // matches the current `literal` vector as a slice with conditions
        // nested in for ==, != and identifiers/keywords
        let result = match self.ch.as_slice() {
            [AsciiChar::Equal] => {
                if [self.peek_char()] == [AsciiChar::Equal] {
                    self.read_char();
                    (TokenType::EQ, vec![AsciiChar::Equal, AsciiChar::Equal])
                } else {
                    (TokenType::ASSIGN, vec![AsciiChar::Equal])
                }
            }
            [AsciiChar::Plus] => (TokenType::PLUS, vec![AsciiChar::Plus]),
            [AsciiChar::Minus] => (TokenType::MINUS, vec![AsciiChar::Minus]),
            [AsciiChar::Exclamation] => {
                if [self.peek_char()] == [AsciiChar::Equal] {
                    self.read_char();
                    (
                        TokenType::NotEq,
                        vec![AsciiChar::Exclamation, AsciiChar::Equal],
                    )
                } else {
                    (TokenType::BANG, vec![AsciiChar::Exclamation])
                }
            }
            [AsciiChar::Slash] => (TokenType::SLASH, vec![AsciiChar::Slash]),
            [AsciiChar::Asterisk] => (TokenType::ASTERISK, vec![AsciiChar::Asterisk]),
            [AsciiChar::LessThan] => (TokenType::LT, vec![AsciiChar::LessThan]),
            [AsciiChar::GreaterThan] => (TokenType::GT, vec![AsciiChar::GreaterThan]),
            [AsciiChar::ParenOpen] => (TokenType::LPAREN, vec![AsciiChar::ParenOpen]),
            [AsciiChar::ParenClose] => (TokenType::RPAREN, vec![AsciiChar::ParenClose]),
            [AsciiChar::CurlyBraceOpen] => (TokenType::LBRACE, vec![AsciiChar::CurlyBraceOpen]),
            [AsciiChar::CurlyBraceClose] => (TokenType::RBRACE, vec![AsciiChar::CurlyBraceClose]),
            [AsciiChar::Comma] => (TokenType::COMMA, vec![AsciiChar::Comma]),
            [AsciiChar::Semicolon] => (TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            [AsciiChar::Null] => (TokenType::EOF, vec![AsciiChar::Null]),
            _ => {
                default = true;
                if self.ch[0].is_ascii_alphanumeric() {
                    let literal = self.read_identifier();
                    (Token::look_up_ident(literal.clone()), literal)
                } else {
                    self.read_char();
                    (TokenType::ILLEGAL, vec![AsciiChar::Null])
                }
            }
        };

        if !default {
            self.read_char();
        }

        result
    }

    /// Advances past whitespace and returns the next `Token`, after receiving
    /// a match from `match_token_type()`.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - a mutable reference to the Lexer construct being used
    pub fn next_token(&mut self) -> Token {
        while self.ch[0].is_ascii_whitespace() {
            self.read_char();
        }
        let (token_type, literal) = self.match_token_type();
        Token::new(token_type, literal)
    }
}

//
// Tests
//

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
        assert_eq!(ascii_chars.len(), literal.len());
        assert_eq!(ascii_chars.as_slice(), literal.as_slice());
    }

    #[test]
    fn peek_char() {
        let l: Lexer = Lexer::new("==let five cat".to_string());
        assert_eq!(l.peek_char(), AsciiChar::Equal)
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
        let input: String = String::from(
            "let five = 5;
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
",
        );

        let tests: Vec<Token> = vec![
            //line 1
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
            // line 2
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
            // line 3
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
            // line 4
            Token::new(TokenType::IDENT, vec![AsciiChar::x]),
            Token::new(TokenType::PLUS, vec![AsciiChar::Plus]),
            Token::new(TokenType::IDENT, vec![AsciiChar::y]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            // line 5  };
            Token::new(TokenType::RBRACE, vec![AsciiChar::CurlyBraceClose]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            // line 6  let result = add(five, ten);
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
            // line 7:  !-/*5;
            Token::new(TokenType::BANG, vec![AsciiChar::Exclamation]),
            Token::new(TokenType::MINUS, vec![AsciiChar::Minus]),
            Token::new(TokenType::SLASH, vec![AsciiChar::Slash]),
            Token::new(TokenType::ASTERISK, vec![AsciiChar::Asterisk]),
            Token::new(TokenType::INT, vec![AsciiChar::_5]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            // line 8:  5 < 10 > 5;
            Token::new(TokenType::INT, vec![AsciiChar::_5]),
            Token::new(TokenType::LT, vec![AsciiChar::LessThan]),
            Token::new(TokenType::INT, vec![AsciiChar::_1, AsciiChar::_0]),
            Token::new(TokenType::GT, vec![AsciiChar::GreaterThan]),
            Token::new(TokenType::INT, vec![AsciiChar::_5]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            // line 9
            // line 10:  if (5 < 10) {
            Token::new(TokenType::IF, vec![AsciiChar::i, AsciiChar::f]),
            Token::new(TokenType::LPAREN, vec![AsciiChar::ParenOpen]),
            Token::new(TokenType::INT, vec![AsciiChar::_5]),
            Token::new(TokenType::LT, vec![AsciiChar::LessThan]),
            Token::new(TokenType::INT, vec![AsciiChar::_1, AsciiChar::_0]),
            Token::new(TokenType::RPAREN, vec![AsciiChar::ParenClose]),
            Token::new(TokenType::LBRACE, vec![AsciiChar::CurlyBraceOpen]),
            // line 11:  return true;
            Token::new(
                TokenType::RETURN,
                vec![
                    AsciiChar::r,
                    AsciiChar::e,
                    AsciiChar::t,
                    AsciiChar::u,
                    AsciiChar::r,
                    AsciiChar::n,
                ],
            ),
            Token::new(
                TokenType::TRUE,
                vec![AsciiChar::t, AsciiChar::r, AsciiChar::u, AsciiChar::e],
            ),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            // line 12:  } else {
            Token::new(TokenType::RBRACE, vec![AsciiChar::CurlyBraceClose]),
            Token::new(
                TokenType::ELSE,
                vec![AsciiChar::e, AsciiChar::l, AsciiChar::s, AsciiChar::e],
            ),
            Token::new(TokenType::LBRACE, vec![AsciiChar::CurlyBraceOpen]),
            // line 13:  return false;
            Token::new(
                TokenType::RETURN,
                vec![
                    AsciiChar::r,
                    AsciiChar::e,
                    AsciiChar::t,
                    AsciiChar::u,
                    AsciiChar::r,
                    AsciiChar::n,
                ],
            ),
            Token::new(
                TokenType::FALSE,
                vec![
                    AsciiChar::f,
                    AsciiChar::a,
                    AsciiChar::l,
                    AsciiChar::s,
                    AsciiChar::e,
                ],
            ),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            // line 14:  }
            Token::new(TokenType::RBRACE, vec![AsciiChar::CurlyBraceClose]),
            // line 15:
            // line 16: 10 == 10;
            Token::new(TokenType::INT, vec![AsciiChar::_1, AsciiChar::_0]),
            Token::new(TokenType::EQ, vec![AsciiChar::Equal, AsciiChar::Equal]),
            Token::new(TokenType::INT, vec![AsciiChar::_1, AsciiChar::_0]),
            Token::new(TokenType::SEMICOLON, vec![AsciiChar::Semicolon]),
            // line 17:  10 != 9;
            Token::new(TokenType::INT, vec![AsciiChar::_1, AsciiChar::_0]),
            Token::new(
                TokenType::NotEq,
                vec![AsciiChar::Exclamation, AsciiChar::Equal],
            ),
            Token::new(TokenType::INT, vec![AsciiChar::_9]),
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
