use token::token;

pub enum Node {
    Program(Vec<Statement>),
}

impl Node {
    // TODO: change to debug impl
    pub fn token_literal(&self) -> Option<String> {
        match self {
            Node::Program(statements) => statements.get(0).map(|s| s.token_literal()),
        }
    }
}

pub enum Statement {
    Let(Expression, Expression),
}

impl Statement {
    // TODO: change to debug impl
    pub fn token_literal(&self) -> String {
        match self {
            Statement::Let(_, _) => format!("{:?}", token::TokenKind::Let),
        }
    }
}

pub enum Expression {
    Identifier(String),
}

impl Expression {
    // TODO: change to debug impl
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(_) => String::from("Ident"),
        }
    }
}
