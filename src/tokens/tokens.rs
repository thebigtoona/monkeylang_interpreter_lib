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
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Vec<u8>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        literal: Vec<u8>,
    ) -> Token {
        Token {
            token_type,
            literal
        }
    }
    
    // pub fn look_up_ident(literal: Vec<u8>) -> TokenType {
    //     let l = str::from_utf8(literal);

    //     match l {
    //         "let" => TokenType::LET,
    //         "fn" => TokenType::FUNCTION,
    //         l.parse.unwrap_or(false) => TokenType::INT,
    //     }

    // }
}



#[cfg(test)]
mod tests {
    use super::*;
    fn new_token() {
        let token = Token::new(TokenType::PLUS, vec![b'+']);

        assert_eq!(token, Token { token_type: TokenType::PLUS, literal: vec![b'+']});
    }
}