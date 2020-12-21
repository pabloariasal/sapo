use super::token::Token;

#[derive(Debug)]
pub enum Expression {
    BinaryExpression,
    UnaryExpression,
    IntegerLiteral,
    BooleanLiteral,
    StringLiteral,
    Grouping,
}

pub struct BinaryExpression {
    pub token: Token,
    left: Expression,
    right: Expression,
}

pub struct UnaryExpression {
    token: Token,
    right: Expression,
}

pub struct IntegerLiteral {
    token: Token,
    value: i32,
}

pub struct BooleanLiteral {
    token: Token,
    value: bool,
}

pub struct StringLiteral {
    token: Token,
    value: String,
}

pub struct Grouping {
    expression: Expression,
}
