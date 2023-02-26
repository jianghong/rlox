use crate::rlox::token::Token;

pub enum Expr {
    Literal(Option<String>),
    Binary {
        left:Box<Expr>,
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
    Ternary {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expr::Literal(value) => visitor.visit_literal(value),
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Ternary { condition, then_branch, else_branch } => visitor.visit_ternary(condition, then_branch, else_branch),
        }
    }
}

pub trait Visitor<T> {
    fn visit_literal(&self, value: &Option<String>) -> T;
    fn visit_binary(&self, left: &Expr, operation: &Token, right: &Expr) -> T;
    fn visit_grouping(&self, expression: &Expr) -> T;
    fn visit_unary(&self, operator: &Token, right: &Expr) -> T;
    fn visit_ternary(&self, condition: &Expr, then_branch: &Expr, else_branch: &Expr) -> T;
}


