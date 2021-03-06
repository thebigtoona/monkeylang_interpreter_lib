use ascii::*;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    // Identifiers + Literals
    IDENT,
    INT,
    // Operators
    ASSIGN, 
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    // comparison operators
    LT,
    GT,
    EQ,
    NotEq,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN
}

/// represents a monkey lang Token for the interpreter to parse
/// 
/// # Parameters
/// 
/// * `token_type` - `TokenType` - item describing what type of identifier the
/// lexer has read
/// 
/// * `literal` - `Vec<AsciiChar>` - the characters parsed to create the 
/// identifier/keyword read by the lexer. Represented as a vector of `AsciiChar` 
/// items.
/// 
#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Vec<AsciiChar>,
}

impl Token {
    /// takes in a token type, & literal value as a vector of AsciiChar items, 
    /// and returns a new Token
    pub fn new(token_type: TokenType, literal: Vec<AsciiChar>) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    /// takes in a vector of asciichar items representing a literal value from 
    /// input, and returns a tokentype matching the value given.
    pub fn look_up_ident(literal: Vec<AsciiChar>) -> TokenType {
        if literal[0].is_ascii_digit() {
            TokenType::INT
        } else {
            match literal.as_slice() {
                [AsciiChar::f, AsciiChar::n] => TokenType::FUNCTION,
                [AsciiChar::l, AsciiChar::e, AsciiChar::t] => TokenType::LET,
                [AsciiChar::t, AsciiChar::r, AsciiChar::u, AsciiChar::e] => TokenType::TRUE,
                [AsciiChar::f, AsciiChar::a, AsciiChar::l, AsciiChar::s, AsciiChar::e] => TokenType::FALSE,
                [AsciiChar::i, AsciiChar::f] => TokenType::IF,
                [AsciiChar::e, AsciiChar::l, AsciiChar::s, AsciiChar::e] => TokenType::ELSE,
                [AsciiChar::r, AsciiChar::e, AsciiChar::t, AsciiChar::u, AsciiChar::r, AsciiChar::n] => TokenType::RETURN,
                _ => TokenType::IDENT,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_token() {
        let token = Token::new(TokenType::PLUS, vec![AsciiChar::Plus]);

        assert_eq!(
            token,
            Token {
                token_type: TokenType::PLUS,
                literal: vec![AsciiChar::Plus]
            }
        );
    }

    #[test]
    fn look_up_ident() {
        let number_test = vec![AsciiChar::_3, AsciiChar::_6];
        let string_test = vec![AsciiChar::f, AsciiChar::i, AsciiChar::v, AsciiChar::e];
        let keyword_test = vec![AsciiChar::l, AsciiChar::e, AsciiChar::t];
        assert_eq!(Token::look_up_ident(number_test), TokenType::INT);
        assert_eq!(Token::look_up_ident(string_test), TokenType::IDENT);
        assert_eq!(Token::look_up_ident(keyword_test), TokenType::LET);
    }
}
