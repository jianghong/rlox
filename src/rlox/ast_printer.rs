use super::token::Token;
use super::token_type::TokenType;
use super::expr::*;

pub fn main() {
    let test_expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal(Some("123".to_string())))
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal(Some("45.67".to_string())))
        })
    };

    let ast_printer = AstPrinter::new();
    println!("{}", ast_printer.print(&test_expr));
}

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &String, exprs: &Vec<&Expr>) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(&name);
        for expr in exprs {
            result.push_str(" ");
            result.push_str(&expr.accept(self));
        }
        result.push_str(")");
        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &vec![left, right])
    }

    fn visit_grouping(&self, expr: &Expr ) -> String {
        self.parenthesize(&"group".to_string(), &vec![expr])
    }

    fn visit_literal(&self, value: &Option<String>) -> String {
        match value {
            Some(value) => value.clone(),
            None => "nil".to_string()
        }
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &vec![right])
    }
}