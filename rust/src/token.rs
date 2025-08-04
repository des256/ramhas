use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]

pub enum Token {
    Eof,
    Integer(i64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Equal,
    Fn,
    Return,
    Int,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Eof => write!(f, "EOF"),
            Token::Integer(value) => write!(f, "{}", value),
            Token::Identifier(value) => write!(f, "{}", value),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Equal => write!(f, "="),
            Token::Fn => write!(f, "fn"),
            Token::Return => write!(f, "return"),
            Token::Int => write!(f, "int"),
        }
    }
}
