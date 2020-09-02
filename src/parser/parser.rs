use crate::tokens::tokens::Token;
use crate::lexer::lexer::Lexer;
use crate::ast::ast::*;

struct Parser {
    lexer: Lexer;
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    // Create new Parser
    new(lexer: &mut Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }
}

impl Parser {
    next_token(&mut Self) {
        Self.current_token = Self.peek_token;
        Self.peek_token = Self.lexer.next_token();
    }
    parse_program(&mut Self) -> Result<Program> {
        let mut program: Program = Program { statements: vec![] };
        while Self.lexer.current_token.token_type != TokenType::EOF {
            let statement: Option = Self.parse_statement();
            match statement {
                Ok(stmt) => 
                Err(err) => 
            }
        }
    }
    parse_statement(){}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let test_input: String = "let x = 5;
let foobar = 12345;
let y = 5;".to_string();

        let mut lexer = Lexer.new(test_input);
        let mut parser = Parser.new(&mut lexer);
        let mut program = parser.parse_program();

        match program {
            None => eprintln!("parse_program returned None")
            Some(data) => println!("parse_program data: {:?}", data)
        };

        assert_eq!(program.statements.len(), 3);
    }
}