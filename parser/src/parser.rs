use ast::ast;
use lexer::lexer::{LexError, LexErrorKind, Lexer};
use token::token::Token;

pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Result<Self, LexError> {
        let mut p = Parser {
            l,
            cur_token: None,
            peek_token: None,
        };
        p.next_token()?;
        p.next_token()?;
        Ok(p)
    }

    fn next_token(&mut self) -> Result<(), LexError> {
        self.cur_token = self.peek_token.clone();
        match self.l.next_token() {
            Ok(t) => {
                self.peek_token = Some(t);
                Ok(())
            }
            Err(LexErrorKind::Eof) => {
                self.peek_token = None;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn parse_program(&self) -> &ast::Program {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::{ast::*, Lexer, Parser};
    #[test]
    fn test_let_stmt() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;
        let mut l = Lexer::new(input);
        let p = Parser::new(&mut l).unwrap();
        let program = p.parse_program();

        assert_eq!(program.len(), 3);

        let expected = vec!["x", "y", "foobar"];
        for n in 0..program.len() {
            assert_let_stmt(&program[n], expected[n]);
        }
    }

    fn assert_let_stmt(actual: &Statement, expected_v: &str) {
        assert_eq!(&actual.token_literal(), "Let");
        match actual {
            Statement::Let(name, _val) => {
                assert_eq!(name.token_literal(), expected_v);
                return;
            }
            _ => panic!(format!("unexpected statement: {:?}", actual)),
        }
    }
}
