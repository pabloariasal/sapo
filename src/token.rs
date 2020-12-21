use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single character tokens
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Bang,

    // Arithmetic operators
    Minus,
    Plus,
    Dot,
    Slash,

    // Prefixed tokens (!=, ==, <=, etc)
    Assignment,
    Equals,
    BangEquals,
    Greater,
    GreaterEquals,
    Smaller,
    SmallerEquals,

    // Keywords
    If,

    Identifier,
    // Literals
    IntegerLiteral,
    StringLiteral,
    BooleanLiteral,

    // Special tokens
    InvalidToken,
    EOF,
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Token {
        Token { token_type, lexeme }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}
