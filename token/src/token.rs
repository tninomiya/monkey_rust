#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    token_type: TokenKind,
}

impl Token {
    pub fn illegal() -> Self {
        Token {
            token_type: TokenKind::Illegal,
        }
    }

    pub fn eof() -> Self {
        Token {
            token_type: TokenKind::Eof,
        }
    }

    pub fn ident(s: &str) -> Self {
        Token {
            token_type: TokenKind::Ident(s.to_string()),
        }
    }

    pub fn int(n: i64) -> Self {
        Token {
            token_type: TokenKind::Int(n),
        }
    }

    pub fn assign() -> Self {
        Token {
            token_type: TokenKind::Assign,
        }
    }

    pub fn plus() -> Self {
        Token {
            token_type: TokenKind::Plus,
        }
    }

    pub fn comma() -> Self {
        Token {
            token_type: TokenKind::Comma,
        }
    }

    pub fn semicolon() -> Self {
        Token {
            token_type: TokenKind::Semicolon,
        }
    }

    pub fn lparen() -> Self {
        Token {
            token_type: TokenKind::LParen,
        }
    }

    pub fn rparen() -> Self {
        Token {
            token_type: TokenKind::RParen,
        }
    }

    pub fn lbrace() -> Self {
        Token {
            token_type: TokenKind::LBrace,
        }
    }

    pub fn rbrace() -> Self {
        Token {
            token_type: TokenKind::RBrace,
        }
    }

    pub fn keyword_function() -> Self {
        Token {
            token_type: TokenKind::Function,
        }
    }

    pub fn keyword_let() -> Self {
        Token {
            token_type: TokenKind::Let,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Illegal,
    Eof,

    // identifier + literal
    // add, foobar, x, y,...
    Ident(String),
    // 123456
    Int(i64),

    // operator
    // "="
    Assign,
    // "+"
    Plus,

    // delimiter
    // ","
    Comma,
    // ";"
    Semicolon,

    // "("
    LParen,
    // ")"
    RParen,
    // "{"
    LBrace,
    // "}"
    RBrace,

    // keyword
    // function
    Function,
    // let
    Let,
}
