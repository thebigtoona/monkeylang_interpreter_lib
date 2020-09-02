use crate::lexer::lexer::Lexer;
use crate::tokens::tokens::{Token, TokenType};
use std::io::*;

const PROMPT: &[u8] = b">> ";

pub fn start(stdin: Stdin, stdout: Stdout) {
    let mut buffer = String::new();
    let mut out_handle = stdout.lock();
    let mut in_handle = stdin.lock();

    println!("Welcome to monkey lang!");
    println!("Ctl+C to stop the interpreter\n");

    loop {
        out_handle.write(PROMPT).unwrap();
        out_handle.flush().unwrap();
        match in_handle.read_line(&mut buffer) {
            Ok(_buf) => {
                let mut l = Lexer::new(buffer.clone());
                let mut current_token: Token = l.next_token();

                while current_token.token_type != TokenType::EOF {
                    println!("{:?}", current_token);
                    current_token = l.next_token();
                }

                out_handle.flush().unwrap();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
        buffer = String::new();
    }
}
