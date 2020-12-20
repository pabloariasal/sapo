#[derive(Debug, PartialEq, Clone)]
pub enum Token
{
    // Single character tokens
    Semicolon,
    LeftParen,
    RightParen,

    // Arithmetic operators
    Minus,
    Plus,
    Dot,
    Slash,

    // Prefixed tokens (!=, ==, <=, etc)
    Assignment,
    Equals,

    // Keywords
    If,

    // Literals
    Identifier(String),
    IntegerLiteral(String),
    StringLiteral(String),

    // Special tokens
    InvalidToken(String),
    EOF,
}
