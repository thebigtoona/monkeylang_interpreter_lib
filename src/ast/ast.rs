use crate::tokens::tokens::Token;

trait Node {
    fn token_literal() -> String;
}

trait Expression {
    fn expression_node;
}

trait Statement {
    fn statement_node(&Self);
}

impl<T: Statement> Node for T {}
impl<T: Expression> Node for T {}


// Program
//
struct Program {
    statements: Vec<Statement>,
}


impl Node for Program {
    fn token_literal (&Self) -> String {
        match self.statements.len() > 0 {
            true => self.statements[0].token_literal()
            _ => "".to_string()
        }
    }
}


// LetStatement
//
struct LetStatement {
    token: Token,
    name: *Identifier,
    value: Expression,
}

impl Node, Statement for LetStatement {
    // Node
    fn token_literal(&Self) -> String {
        Self.token.literal.to_string()
    }
    // Statement
    fn statement_node(&Self);
}


// Identifier
//
struct Identifier {
    token: Token,
    value: String,
}

impl Node, Expression for Identifier {
    // Node 
    fn token_literal(&Self) -> String {
        Self.token.literal.to_string()
    }
    // Expression
    fn expression_node(&Self);
}
