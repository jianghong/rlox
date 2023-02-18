use crate::rlox::token::Token;

pub trait Expr<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

pub struct Literal {
    pub value: Option<String>,
}

impl<T> Expr<T> for Literal {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_literal(self)
    }
}

pub struct Binary<T> {
    pub left: Box<dyn Expr<T>>,
    pub operator: Token,
    pub right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Binary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary(self)
    }
}

pub struct Grouping<T> {
    pub expression: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Grouping<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_grouping(self)
    }
}

pub struct Unary<T> {
    pub operator: Token,
    pub right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Unary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_unary(self)
    }
}

pub trait Visitor<T> {
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_binary(&self, expr: &Binary<T>) -> T;
    fn visit_grouping(&self, expr: &Grouping<T>) -> T;
    fn visit_unary(&self, expr: &Unary<T>) -> T;
}


