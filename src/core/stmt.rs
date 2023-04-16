use super::expr::Expr;

pub struct Void;

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
        }
    }
}
pub trait Visitor<T> {
    fn visit_expression(&self, expr: &Expr) -> T;
    fn visit_print(&self, expr: &Expr) -> T;
}
