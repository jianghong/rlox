use crate::rlox::token::Token;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

#[derive(Debug, Clone)]

pub enum Value {
    Nil,
    True,
    False,
    Number(f64),
    String(String),
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Value::Number(value) => {
                if let Value::Number(other) = other {
                    Value::Number(value + other)
                } else {
                    panic!("Applying '+' operator to a non number.")
                }
            }
            Value::String(value) => {
                if let Value::String(other) = other {
                    Value::String(format!("{}{}", value, other))
                } else {
                    panic!("Applying '+' operator to a non string.")
                }
            }
            _ => panic!("Applying '+' operator to value that is not applicable."),
        } 
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            Value::Number(value) => {
                if let Value::Number(other) = other {
                    Value::Number(value - other)
                } else {
                    panic!("Applying '-' operator to a non number.")
                }
            }
            _ => panic!("Applying '-' operator to a non number."),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            Value::Number(value) => {
                if let Value::Number(other) = other {
                    Value::Number(value * other)
                } else {
                    panic!("Applying '*' operator to a non number.")
                }
            }
            _ => panic!("Applying '*' operator to a non number."),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match self {
            Value::Number(value) => {
                if let Value::Number(other) = other {
                    Value::Number(value / other)
                } else {
                    panic!("Applying '/' operator to a non number.")
                }
            }
            _ => panic!("Applying '/' operator to a non number."),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Value::Number(value) => {
                if let Value::Number(other) = other {
                    value.partial_cmp(other).unwrap()
                } else {
                    panic!("Applying '>' operator to a non number.")
                }
            }
            _ => panic!("Applying '>' operator to a non number."),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for Value {}

impl Value {
    pub fn to_number(value: &str) -> Value {
        let value = value.parse::<f64>().unwrap();
        Value::Number(value)
    }

    pub fn to_string(value: &str) -> Value {
        Value::String(value.to_string())
    }

}

pub enum Expr {
    Literal(Option<Value>),
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
    fn visit_literal(&self, value: &Option<Value>) -> T;
    fn visit_binary(&self, left: &Expr, operation: &Token, right: &Expr) -> T;
    fn visit_grouping(&self, expression: &Expr) -> T;
    fn visit_unary(&self, operator: &Token, right: &Expr) -> T;
    fn visit_ternary(&self, condition: &Expr, then_branch: &Expr, else_branch: &Expr) -> T;
}


