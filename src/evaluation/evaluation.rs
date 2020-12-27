use crate::ast::Expression;
use crate::token::{Token, TokenType};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    String(String),
}

type EvalResult = Result<Object, String>;

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{}", value),
            Object::String(value) => write!(f, "\"{}\"", value),
            Object::Boolean(value) => write!(f, "{}", value),
        }
    }
}

pub fn evaluate(ast: &Expression) -> EvalResult {
    match ast {
        Expression::IntegerLiteral { token: _, value } => Ok(Object::Integer(*value)),
        Expression::BooleanLiteral { token: _, value } => Ok(Object::Boolean(*value)),
        Expression::StringLiteral { token: _, value } => Ok(Object::String(value.clone())),
        Expression::Grouping { token: _, expr } => evaluate(&*expr),
        Expression::UnaryExpression { token, right } => evaluate_unary_expression(&token, right),
        Expression::BinaryExpression { token, left, right } => {
            evaluate_binary_expression(&token, left, right)
        }
    }
}

fn evaluate_unary_expression(token: &Token, right: &Expression) -> EvalResult {
    let right = evaluate(right)?;
    match token.token_type {
        TokenType::Bang => {
            if let Object::Boolean(value) = right {
                Ok(Object::Boolean(!value))
            } else {
                Err(error("Invalid operand for '!', expected boolean expression", token))
            }
        }
        TokenType::Minus => {
            if let Object::Integer(value) = right {
                Ok(Object::Integer(-value))
            } else {
                Err(error("Invalid operand for '-', expected integer expression", token))
            }
        }
        _ => Err("Unreachable".to_string()),
    }
}

fn evaluate_binary_expression(token: &Token, left: &Expression, right: &Expression) -> EvalResult {
    let left = evaluate(left)?;
    let right = evaluate(right)?;

    match token.token_type {
        TokenType::Minus => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l - r)),
            _ => Err(error("Invalid operands for '-'", token)),
        },
        TokenType::Plus => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
            _ => Err(error("Invalid operands for '+'", token)),
        },
        TokenType::Star => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
            _ => Err(error("Invalid operands for '*'", token)),
        },
        TokenType::Slash => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l / r)),
            _ => Err(error("Invalid operands for '/'", token)),
        },
        TokenType::Greater => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l > r)),
            _ => Err(error("Invalid operands for '>'", token)),
        },
        TokenType::GreaterEquals => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l >= r)),
            _ => Err(error("Invalid operands for '>='", token)),
        },
        TokenType::SmallerEquals => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l <= r)),
            _ => Err(error("Invalid operands for '<='", token)),
        },
        TokenType::Smaller => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l < r)),
            _ => Err(error("Invalid operands for '<'", token)),
        },
        TokenType::Equals => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l == r)),
            (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l == r)),
            _ => Err(error("Invalid operands for '=='", token)),
        },
        TokenType::BangEquals => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l != r)),
            (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l != r)),
            _ => Err(error("Invalid operands for '!='", token)),
        },
        _ => Err("Unreachable".to_string()),
    }
}

fn error(msg: &str, token: &Token) -> String {
    if let TokenType::EOF = token.token_type {
        return format!("Error at end of file: {}", msg);
    }
    format!("Error at line {}: {}", token.line, msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_boolean_literal() {
        let result = evaluate(&Expression::BooleanLiteral {
            token: token(TokenType::BooleanLiteral),
            value: false,
        })
        .unwrap();
        assert_eq!(result, Object::Boolean(false));
    }

    #[test]
    fn evaluate_arithmetic_expression() {
        // 3 - 30 / 6
        let ast = Expression::BinaryExpression {
            token: token(TokenType::Minus),
            left: Box::new(Expression::IntegerLiteral {
                token: token(TokenType::IntegerLiteral),
                value: 3,
            }),
            right: Box::new(integer_binary_expr(TokenType::Slash, 30, 6)),
        };
        let result = evaluate(&ast).unwrap();
        assert_eq!(result, Object::Integer(-2));
    }

    #[test]
    fn evaluate_integer_equality() {
        let result = evaluate(&integer_binary_expr(TokenType::Equals, 30, 30)).unwrap();
        assert_eq!(result, Object::Boolean(true));

        let result = evaluate(&integer_binary_expr(TokenType::Equals, 34, 30)).unwrap();
        assert_eq!(result, Object::Boolean(false));

        let result = evaluate(&integer_binary_expr(TokenType::BangEquals, 30, 30)).unwrap();
        assert_eq!(result, Object::Boolean(false));

        let result = evaluate(&integer_binary_expr(TokenType::BangEquals, 34, 30)).unwrap();
        assert_eq!(result, Object::Boolean(true));
    }

    #[test]
    fn evaluate_bool_equality() {
        let result = evaluate(&bool_binary_expr(TokenType::Equals, true, true)).unwrap();
        assert_eq!(result, Object::Boolean(true));

        let result = evaluate(&bool_binary_expr(TokenType::Equals, false, false)).unwrap();
        assert_eq!(result, Object::Boolean(true));

        let result = evaluate(&bool_binary_expr(TokenType::Equals, true, false)).unwrap();
        assert_eq!(result, Object::Boolean(false));

        let result = evaluate(&bool_binary_expr(TokenType::BangEquals, true, false)).unwrap();
        assert_eq!(result, Object::Boolean(true));

        let result = evaluate(&bool_binary_expr(TokenType::BangEquals, false, false)).unwrap();
        assert_eq!(result, Object::Boolean(false));
    }

    #[test]
    fn evaluate_integer_comparison() {
        let result = evaluate(&integer_binary_expr(TokenType::Smaller, 30, 30)).unwrap();
        assert_eq!(result, Object::Boolean(false));

        let result = evaluate(&integer_binary_expr(TokenType::SmallerEquals, 30, 30)).unwrap();
        assert_eq!(result, Object::Boolean(true));

        let result = evaluate(&integer_binary_expr(TokenType::Greater, 30, 30)).unwrap();
        assert_eq!(result, Object::Boolean(false));

        let result = evaluate(&integer_binary_expr(TokenType::GreaterEquals, 30, 30)).unwrap();
        assert_eq!(result, Object::Boolean(true));
    }

    #[test]
    fn evaluate_grouping() {
        // (12 - 10) * 8
        let ast = Expression::BinaryExpression {
            token: token(TokenType::Star),
            left: Box::new(Expression::Grouping {
                token: token(TokenType::LeftBrace),
                expr: Box::new(integer_binary_expr(TokenType::Minus, 12, 10)),
            }),
            right: Box::new(Expression::IntegerLiteral {
                token: token(TokenType::IntegerLiteral),
                value: 8,
            }),
        };
        let result = evaluate(&ast).unwrap();
        assert_eq!(result, Object::Integer(16));
    }

    #[test]
    #[should_panic(expected = "Invalid operand for '!'")]
    fn wrong_bang_operand() {
        let ast = Expression::UnaryExpression {
            token: token(TokenType::Bang),
            right: Box::new(Expression::IntegerLiteral {
                token: token(TokenType::IntegerLiteral),
                value: 0,
            }),
        };
        let result = evaluate(&ast);
        panic_on_error(result);
    }

    #[test]
    #[should_panic(expected = "Invalid operand for '-'")]
    fn wrong_minus_operand() {
        let ast = Expression::UnaryExpression {
            token: token(TokenType::Minus),
            right: Box::new(Expression::BooleanLiteral {
                token: token(TokenType::BooleanLiteral),
                value: false,
            }),
        };
        let result = evaluate(&ast);
        panic_on_error(result);
    }

    #[test]
    #[should_panic(expected = "Invalid operands for '>='")]
    fn invalid_operands_greater_equals() {
        let result = evaluate(&bool_binary_expr(TokenType::GreaterEquals, false, false));
        panic_on_error(result);
    }

    #[test]
    #[should_panic(expected = "Invalid operands for '>'")]
    fn invalid_operands_greater() {
        let result = evaluate(&bool_binary_expr(TokenType::Greater, false, false));
        panic_on_error(result);
    }

    #[test]
    #[should_panic(expected = "Invalid operands for '<'")]
    fn invalid_operands_smaller() {
        let result = evaluate(&bool_binary_expr(TokenType::Smaller, false, false));
        panic_on_error(result);
    }

    #[test]
    #[should_panic(expected = "Invalid operands for '<='")]
    fn invalid_operands_smaller_equals() {
        let result = evaluate(&bool_binary_expr(TokenType::SmallerEquals, false, false));
        panic_on_error(result);
    }

    fn token(token_type: TokenType) -> Token {
        Token::new(token_type, String::new(), -1)
    }

    fn panic_on_error(result: EvalResult) {
        if let Err(msg) = result {
            panic!(msg);
        }
    }

    fn integer_binary_expr(token_type: TokenType, left: i32, right: i32) -> Expression {
        Expression::BinaryExpression {
            token: token(token_type),
            left: Box::new(Expression::IntegerLiteral {
                token: token(TokenType::IntegerLiteral),
                value: left,
            }),
            right: Box::new(Expression::IntegerLiteral {
                token: token(TokenType::IntegerLiteral),
                value: right,
            }),
        }
    }

    fn bool_binary_expr(token_type: TokenType, left: bool, right: bool) -> Expression {
        Expression::BinaryExpression {
            token: token(token_type),
            left: Box::new(Expression::BooleanLiteral {
                token: token(TokenType::BooleanLiteral),
                value: left,
            }),
            right: Box::new(Expression::BooleanLiteral {
                token: token(TokenType::BooleanLiteral),
                value: right,
            }),
        }
    }
}
