use std::iter::Peekable;
use std::str::Chars;
use token::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    pos: usize,
    read_pos: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lex = Lexer {
            input: input.chars().peekable(),
            pos: 0,
            read_pos: 0,
            ch: None,
        };
        lex.read_char();
        lex
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            Some('=') => Token::assign(),
            Some(';') => Token::semicolon(),
            Some('(') => Token::lparen(),
            Some(')') => Token::rparen(),
            Some(',') => Token::comma(),
            Some('+') => Token::plus(),
            Some('{') => Token::lbrace(),
            Some('}') => Token::rbrace(),
            None => Token::eof(),
            _ => unreachable!(),
        };
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        self.ch = self.input.next();
        self.pos = self.read_pos;
        self.read_pos += 1;
    }
}

#[test]
fn new_token_test() {
    let input = "=+(){},;";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token(), Token::assign());
    assert_eq!(lexer.next_token(), Token::plus());
    assert_eq!(lexer.next_token(), Token::lparen());
    assert_eq!(lexer.next_token(), Token::rparen());
    assert_eq!(lexer.next_token(), Token::lbrace());
    assert_eq!(lexer.next_token(), Token::rbrace());
    assert_eq!(lexer.next_token(), Token::comma());
    assert_eq!(lexer.next_token(), Token::semicolon());
}
