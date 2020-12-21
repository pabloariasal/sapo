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

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Token {
        Token {
            token_type,
            lexeme
        }
    }
}
