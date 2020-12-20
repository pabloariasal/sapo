#[derive(Debug, PartialEq, Clone)]
pub enum Token
{
    // Single character tokens
    Semicolon,
    LeftParen,
    RightParen,
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
    False,
    True,

    // Literals
    Identifier(String),
    IntegerLiteral(String),
    StringLiteral(String),

    // Special tokens
    InvalidToken(String),
    EOF,
}
