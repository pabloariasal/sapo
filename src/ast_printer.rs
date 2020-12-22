use super::ast::Expression;

pub fn print_ast(ast: Box<Expression>) -> String {
    let mut buf = String::new();
    print_expression(&*ast, &mut buf);
    buf
}

fn print_expression(ast: &Expression, buf: &mut String) {
    match &*ast {
        Expression::IntegerLiteral { token: _, value } => {
            buf.push_str(&format!("(IntLit {})", value))
        }
        Expression::BooleanLiteral { token: _, value } => {
            buf.push_str(&format!("(BoolLit {})", value))
        }
        Expression::StringLiteral { token: _, value } => {
            buf.push_str(&format!("(StrLit {})", value))
        }
        Expression::Grouping { token: _, expr } => {
            buf.push_str("(Group ");
            print_expression(&*expr, buf);
            buf.push_str(")");
        }
        Expression::UnaryExpression { token, right } => {
            buf.push_str(&format!("({} ", token.lexeme));
            print_expression(&*right, buf);
            buf.push_str(")");
        }
        Expression::BinaryExpression { token, right, left } => {
            buf.push_str(&format!("({} ", token.lexeme));
            print_expression(&*left, buf);
            buf.push_str(" ");
            print_expression(&*right, buf);
            buf.push_str(")");
        }
    }
}
