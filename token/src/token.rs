pub struct Token {
    token_type: TokenKind,
    literal: String,
}

pub enum TokenKind {
    Illegal,
    Eof,

    // identifier + literal
    // add, foobar, x, y,...
    Ident,
    // 123456
    Int,

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
