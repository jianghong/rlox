use super::token::Token;
use super::token_type::TokenType;
use super::expr::*;

pub fn main() {
    let test_expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal(Some(Value::to_number("123"))))
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal(Some(Value::to_number("45.67"))))
        })
    };

    let ast_printer = AstPrinter::new();
    println!("{}", ast_printer.print(&test_expr));
}

pub struct AstPrinter;

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

    fn visit_literal(&self, value: &Option<Value>) -> String {
        if let Some(value) = value {
            match value {
                Value::Number(value) => value.to_string(),
                Value::String(value) => value.to_string(),
                Value::True => "true".to_string(),
                Value::False => "false".to_string(),
                Value::Nil => "nil".to_string(),
            }
        } else {
            "nil".to_string()
        }
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &vec![right])
    }

    fn visit_ternary(&self, condition: &Expr, then_branch: &Expr, else_branch: &Expr) -> String {
        let mut result = String::new();
        result.push_str("(if ");
        result.push_str(&condition.accept(self));
        result.push_str(" then ");
        result.push_str(&then_branch.accept(self));
        result.push_str(" else ");
        result.push_str(&else_branch.accept(self));
        result.push_str(")");
        result
    }
}