use crate::rlox::token::Token;

pub enum Expr {
    Literal(Option<String>),
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expr::Literal(value) => visitor.visit_literal(value),
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
        }
    }
}

pub trait Visitor<T> {
    fn visit_literal(&self, value: &Option<String>) -> T;
    fn visit_binary(&self, left: &Expr, operation: &Token, right: &Expr) -> T;
    fn visit_grouping(&self, expression: &Expr) -> T;
    fn visit_unary(&self, operator: &Token, right: &Expr) -> T;
}


