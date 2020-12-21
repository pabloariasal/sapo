use super::lexer::Lexer;
use crate::ast;

pub fn parse(input: String) -> Result<ast::Expression, String> {
    let mut lexer = Lexer::new(input);
    Err("temp".to_string())
}
