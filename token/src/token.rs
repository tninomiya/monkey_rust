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

    pub fn minus() -> Self {
        Token {
            token_type: TokenKind::Minus,
        }
    }

    pub fn bang() -> Self {
        Token {
            token_type: TokenKind::Bang,
        }
    }

    pub fn asterisk() -> Self {
        Token {
            token_type: TokenKind::Asterisk,
        }
    }

    pub fn slash() -> Self {
        Token {
            token_type: TokenKind::Slash,
        }
    }

    pub fn lt() -> Self {
        Token {
            token_type: TokenKind::Lt,
        }
    }

    pub fn gt() -> Self {
        Token {
            token_type: TokenKind::Gt,
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

    pub fn keyword_true() -> Self {
        Token {
            token_type: TokenKind::True,
        }
    }

    pub fn keyword_false() -> Self {
        Token {
            token_type: TokenKind::False,
        }
    }

    pub fn keyword_if() -> Self {
        Token {
            token_type: TokenKind::If,
        }
    }

    pub fn keyword_else() -> Self {
        Token {
            token_type: TokenKind::Else,
        }
    }

    pub fn keyword_return() -> Self {
        Token {
            token_type: TokenKind::Return,
        }
    }

    pub fn lookup_ident(ident: &str) -> Token {
        match ident {
            "fn" => Token::keyword_function(),
            "let" => Token::keyword_let(),
            "true" => Token::keyword_true(),
            "false" => Token::keyword_false(),
            "if" => Token::keyword_if(),
            "else" => Token::keyword_else(),
            "return" => Token::keyword_return(),
            s => Token::ident(s),
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
    // "-"
    Minus,
    // "!"
    Bang,
    // "*"
    Asterisk,
    // "/"
    Slash,

    // "<"
    Lt,
    // ">"
    Gt,

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
    // true
    True,
    // false
    False,
    // if
    If,
    // elso
    Else,
    // return
    Return,
}
