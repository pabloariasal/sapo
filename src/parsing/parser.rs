use super::lexer::Lexer;
use crate::ast;
use crate::token;

pub fn parse(input: String) -> Result<ast::Expression, String> {
    // let mut lexer = Lexer::new(input);
    // parse_expression(&mut lexer);
    let literal = Box::new(ast::Expression::IntegerLiteral {
        token: token::Token::new(token::TokenType::IntegerLiteral, String::from("999")),
        value: 999,
    });
    Ok(ast::Expression::UnaryExpression {
        token: token::Token::new(token::TokenType::Bang, String::from("!")),
        right: literal,
    })
}

fn parse_expression(lexer: &mut Lexer) -> Result<ast::Expression, String> {
    parse_equality(lexer)
}

fn parse_equality(lexer: &mut Lexer) -> Result<ast::Expression, String> {
    Err(String::from("ks"))
}

// fn parse_comparison(lexer: &mut Lexer) -> Result<ast::Expression, String> {}

// fn parse_term(lexer: &mut Lexer) -> Result<ast::Expression, String> {}

// fn pare_factor(lexer: &mut Lexer) -> Result<ast::Expression, String> {}

// fn parse_unary_operation(lexer: &mut Lexer) -> Result<ast::Expression, String> {}

// fn parse_primary_expr(lexer: &mut Lexer) -> Result<ast::Expression, String> {}

fn match_token<I>(lexer: &mut Lexer, types_to_match: I) -> Option<token::Token>
where
    I: IntoIterator<Item = token::TokenType>,
{
    let next_token = lexer.next_token();
    let found = types_to_match
        .into_iter()
        .find(|t| *t == next_token.token_type);
    match found {
        Some(_) => Some(next_token),
        _ => None,
    }
}
