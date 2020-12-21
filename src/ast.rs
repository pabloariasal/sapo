use super::token::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryExpression {
        token: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryExpression {
        token: Token,
        right: Box<Expression>,
    },
    IntegerLiteral {
        token: Token,
        value: i32,
    },
    BooleanLiteral {
        token: Token,
        value: bool,
    },
    StringLiteral {
        token: Token,
        value: String,
    },
    Grouping {
        token: Token,
        expr: Box<Expression>,
    },
}
