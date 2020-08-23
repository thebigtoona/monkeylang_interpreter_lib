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


pub struct Token {
    pub token_type: TokenType,
    pub literal: Vec<u8>,
}

// impl Token {
    // look_up_ident(literal: Vec<u8>) -> TokenType {
    //     let l = str::from_utf8(literal);

    //     match l {
    //         "let" => TokenType::LET,
    //         "fn" => TokenType::FUNCTION,
    //         l.parse.unwrap_or(false) => TokenType::INT,
    //     }

    // }
// }