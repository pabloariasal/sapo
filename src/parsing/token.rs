#[derive(Debug, PartialEq, Clone)]
pub enum Token
{
    InvalidToken(String),
    Identifier(String),
    IntegerLiteral(String),
    StringLiteral(String),
    Semicolon,
    Assignment,
    Minus,
    If,
    Equals,
    EOF,
}
