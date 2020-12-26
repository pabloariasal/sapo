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
    Star,
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

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Token {
        Token { token_type, lexeme, line }
    }
}
