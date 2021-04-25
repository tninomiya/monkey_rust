use token::token;

pub type Program = Vec<Statement>;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
}

impl Expression {
    // TODO: change to debug impl
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(l) => l.clone(),
        }
    }
}
