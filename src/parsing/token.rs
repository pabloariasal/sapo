#[derive(Debug, PartialEq, Clone)]
pub enum Token
{
    InvalidToken(String),
    Identifier(String),
    IntegerLiteral(String),
    StringLiteral(String),
    Semicolon,
    Assign,
    Minus,
    If,
    Equals,
    EOF,
}
