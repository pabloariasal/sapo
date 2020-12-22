use super::lexer::Lexer;
use crate::ast;
use crate::token::{Token, TokenType};
use std::iter::Peekable;

type ParsedExpressionResult = Result<Box<ast::Expression>, String>;

pub fn parse(input: String) -> ParsedExpressionResult {
    parse_expression(&mut Lexer::new(input).peekable())
}

fn parse_expression<I>(tokens: &mut Peekable<I>) -> ParsedExpressionResult
where
    I: Iterator<Item = Token>,
{
    parse_equality(tokens)
}

fn parse_equality<I>(tokens: &mut Peekable<I>) -> ParsedExpressionResult
where
    I: Iterator<Item = Token>,
{
    const EQUALITY_TOKENS: [TokenType; 2] = [TokenType::Bang, TokenType::BangEquals];
    let mut left = parse_comparison(tokens)?;
    while let Some(token) = match_token(tokens, &EQUALITY_TOKENS) {
        let right = parse_comparison(tokens)?;
        left = Box::new(ast::Expression::BinaryExpression { token, left, right });
    }
    Ok(left)
}

fn parse_comparison<I>(tokens: &mut Peekable<I>) -> ParsedExpressionResult
where
    I: Iterator<Item = Token>,
{
    const COMPARISON_TOKENS: [TokenType; 4] = [
        TokenType::Greater,
        TokenType::GreaterEquals,
        TokenType::Smaller,
        TokenType::SmallerEquals,
    ];
    let mut left = parse_term(tokens)?;
    while let Some(token) = match_token(tokens, &COMPARISON_TOKENS) {
        let right = parse_term(tokens)?;
        left = Box::new(ast::Expression::BinaryExpression { token, left, right });
    }
    Ok(left)
}

fn parse_term<I>(tokens: &mut Peekable<I>) -> ParsedExpressionResult
where
    I: Iterator<Item = Token>,
{
    const TERM_TOKENS: [TokenType; 2] = [TokenType::Minus, TokenType::Plus];
    let mut left = parse_factor(tokens)?;
    while let Some(token) = match_token(tokens, &TERM_TOKENS) {
        let right = parse_factor(tokens)?;
        left = Box::new(ast::Expression::BinaryExpression { token, left, right });
    }
    Ok(left)
}

fn parse_factor<I>(tokens: &mut Peekable<I>) -> ParsedExpressionResult
where
    I: Iterator<Item = Token>,
{
    const FACTOR_TOKENS: [TokenType; 2] = [TokenType::Star, TokenType::Slash];
    let mut left = parse_unary_operation(tokens)?;
    while let Some(token) = match_token(tokens, &FACTOR_TOKENS) {
        let right = parse_unary_operation(tokens)?;
        left = Box::new(ast::Expression::BinaryExpression { token, left, right });
    }
    Ok(left)
}

fn parse_unary_operation<I>(tokens: &mut Peekable<I>) -> ParsedExpressionResult
where
    I: Iterator<Item = Token>,
{
    const UNARY_OPERATORS: [TokenType; 2] = [TokenType::Bang, TokenType::Minus];
    if let Some(token) = match_token(tokens, &UNARY_OPERATORS) {
        // stuff like !! and even -- is allowed by the grammar...
        let right = parse_unary_operation(tokens)?;
        return Ok(Box::new(ast::Expression::UnaryExpression { token, right }));
    }
    parse_primary_expr(tokens)
}

fn parse_primary_expr<I>(tokens: &mut Peekable<I>) -> ParsedExpressionResult
where
    I: Iterator<Item = Token>,
{
    if let Some(token) = match_token(tokens, &[TokenType::IntegerLiteral]) {
        let value = token.lexeme.parse::<i32>().unwrap();
        return Ok(Box::new(ast::Expression::IntegerLiteral { token, value }));
    }
    if let Some(token) = match_token(tokens, &[TokenType::BooleanLiteral]) {
        let value = token.lexeme.parse::<bool>().unwrap();
        return Ok(Box::new(ast::Expression::BooleanLiteral { token, value }));
    }
    if let Some(token) = match_token(tokens, &[TokenType::StringLiteral]) {
        let value = token.lexeme.clone();
        return Ok(Box::new(ast::Expression::StringLiteral { token, value }));
    }
    if let Some(token) = match_token(tokens, &[TokenType::LeftParen]) {
        let expr = parse_expression(tokens)?;
        if let None = match_token(tokens, &[TokenType::RightParen]) {
            return Err(String::from("Expected ')'"));
        };
        return Ok(Box::new(ast::Expression::Grouping { token, expr }));
    }
    Err("Invalid expression".to_string())
}

fn match_token<I>(tokens: &mut Peekable<I>, types_to_match: &[TokenType]) -> Option<Token>
where
    I: Iterator<Item = Token>,
{
    if let Some(next_token) = tokens.peek() {
        if let Some(_) = types_to_match.iter().find(|&t| *t == next_token.token_type) {
            return tokens.next();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_printer;

    #[test]
    fn parse_integer_literal() {
        assert_ast("6", "(IntLit 6)");
    }

    #[test]
    fn parse_string_literal() {
        assert_ast("\"test\"", "(StrLit test)");
    }

    #[test]
    fn parse_boolean_literal() {
        assert_ast("false", "(BoolLit false)");
    }

    #[test]
    fn parse_single_binary_expression() {
        assert_ast("7 >= 8", "(>= (IntLit 7) (IntLit 8))")
    }

    #[test]
    fn parse_unary_expression() {
        assert_ast("-9", "(- (IntLit 9))")
    }

    #[test]
    fn parse_grouping_expression() {
        assert_ast("(9)", "(Group (IntLit 9))")
    }

    #[test]
    fn operator_associativity() {
        assert_ast("7 * 9 * 3", "(* (* (IntLit 7) (IntLit 9)) (IntLit 3))")
    }

    #[test]
    fn operator_precedence() {
        assert_ast("7 * 9 - 3", "(- (* (IntLit 7) (IntLit 9)) (IntLit 3))")
    }

    #[test]
    fn operator_precedence_with_grouping() {
        assert_ast(
            "7 * (9 + 3)",
            "(* (IntLit 7) (Group (+ (IntLit 9) (IntLit 3))))",
        )
    }

    fn assert_ast(input: &str, expected: &str) {
        let ast = parse(String::from(input)).unwrap();
        assert_eq!(ast_printer::print_ast(ast), expected);
    }
}
