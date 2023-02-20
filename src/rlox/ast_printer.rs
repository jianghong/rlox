use super::token::Token;
use super::token_type::TokenType;
use super::expr::*;

pub fn main() {
    let test_expr: Binary<String> = Binary {
        left: Box::new(Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Literal { value: Some("123".to_string()) })
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(Grouping {
            expression: Box::new(Literal { value: Some("45.67".to_string()) })
        })
    };

    let ast_printer = AstPrinter::new();
    println!("{}", ast_printer.print(Box::new(test_expr)));
}

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    pub fn print(&self, expr: Box<dyn Expr<String>>) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &String, exprs: &Vec<&Box<dyn Expr<String>>>) -> String {
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
    fn visit_binary(&self, expr: &Binary<String>) -> String {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.left, &expr.right])
    }

    fn visit_grouping(&self, expr: &Grouping<String>) -> String {
        self.parenthesize(&"group".to_string(), &vec![&expr.expression])
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        if expr.value.is_none() {
            "nil".to_string()
        } else {
            expr.value.clone().unwrap()
        }
    }

    fn visit_unary(&self, expr: &Unary<String>) -> String {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.right])
    }
}