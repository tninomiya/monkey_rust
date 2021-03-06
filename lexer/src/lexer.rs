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

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        let tok = match self.ch {
            Some('=') => Token::assign(),
            Some(';') => Token::semicolon(),
            Some('(') => Token::lparen(),
            Some(')') => Token::rparen(),
            Some(',') => Token::comma(),
            Some('+') => Token::plus(),
            Some('{') => Token::lbrace(),
            Some('}') => Token::rbrace(),
            Some(c) if is_identifier(&c) => {
                let literal = &self.read_identifier()?;
                Token::lookup_ident(literal)
            }
            Some(c) if is_number(&c) => {
                let literal = &self.read_number()?;
                let num: i64 = literal.parse::<i64>().unwrap();
                Token::int(num)
            }
            Some(c) => return Err(LexError::invalid_char(c)),
            None => Token::eof(),
        };
        self.read_char();
        Ok(tok)
    }

    fn read_char(&mut self) {
        self.ch = self.input.next();
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn read_identifier(&mut self) -> Result<String, LexError> {
        self.read_sequence(is_identifier)
    }

    fn read_number(&mut self) -> Result<String, LexError> {
        self.read_sequence(is_number)
    }

    fn read_sequence(&mut self, mut f: impl FnMut(&char) -> bool) -> Result<String, LexError> {
        let mut ident = String::new();
        match self.ch {
            Some(c) if f(&c) => ident.push(self.ch.unwrap()),
            Some(c) => return Err(LexError::invalid_char(c)),
            None => return Err(LexError::eof()),
        }
        while self.input.peek().map_or(false, |c| f(c)) {
            self.read_char();
            ident.push(self.ch.unwrap());
        }
        Ok(ident)
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ') | Some('\n') | Some('\t') | Some('\r') = self.ch {
            self.read_char()
        }
    }
}

fn is_identifier(c: &char) -> bool {
    ('a'..='z').contains(c) || ('A'..='Z').contains(c) || &'_' == c
}

fn is_number(c: &char) -> bool {
    ('0'..='9').contains(c)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

pub type LexError = LexErrorKind;

impl LexError {
    fn invalid_char(c: char) -> Self {
        LexErrorKind::InvalidChar(c)
    }
    fn eof() -> Self {
        LexErrorKind::Eof
    }
}

#[test]
fn new_token_test() {
    let input = "=+(){},;";
    let mut lexer = Lexer::new(input);
    let expected = vec![
        Token::assign(),
        Token::plus(),
        Token::lparen(),
        Token::rparen(),
        Token::lbrace(),
        Token::rbrace(),
        Token::comma(),
        Token::semicolon(),
    ];
    for tok in expected {
        assert_eq!(lexer.next_token(), Ok(tok));
    }

    let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
"#;
    let mut lexer = Lexer::new(input);
    let expected = vec![
        Token::keyword_let(),
        Token::ident("five"),
        Token::assign(),
        Token::int(5),
        Token::semicolon(),
        Token::keyword_let(),
        Token::ident("ten"),
        Token::assign(),
        Token::int(10),
        Token::semicolon(),
        Token::keyword_let(),
        Token::ident("add"),
        Token::assign(),
        Token::keyword_function(),
        Token::lparen(),
        Token::ident("x"),
        Token::comma(),
        Token::ident("y"),
        Token::rparen(),
        Token::lbrace(),
        Token::ident("x"),
        Token::plus(),
        Token::ident("y"),
        Token::semicolon(),
        Token::rbrace(),
        Token::semicolon(),
        Token::keyword_let(),
        Token::ident("result"),
        Token::assign(),
        Token::ident("add"),
        Token::lparen(),
        Token::ident("five"),
        Token::comma(),
        Token::ident("ten"),
        Token::rparen(),
        Token::semicolon(),
    ];
    for tok in expected {
        assert_eq!(lexer.next_token(), Ok(tok));
    }
}
