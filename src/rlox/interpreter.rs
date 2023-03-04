use crate::rlox::expr::*;
use crate::rlox::token::Token;
use crate::rlox::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&self, expression: &Expr) {
        println!("{}", self.stringify(&self.evalute(expression)));
    }

    fn evalute(&self, expression: &Expr) -> Value {
        expression.accept(self)
    }

    fn is_truthy(&self, value: &Value) -> Value {
        match value {
            Value::Nil => Value::False,
            Value::False => Value::False,
            Value::True => Value::True,
            _ => Value::True,
        }
    }
    
    fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Nil => "nil".to_string(),
            Value::True => "true".to_string(),
            Value::False => "false".to_string(),
            Value::Number(value) => value.to_string(),
            Value::String(value) => value.to_string(),
        }
    }
}

impl Visitor<Value> for Interpreter {
    fn visit_literal(&self, value: &Option<Value>) -> Value {
        match value {
            Some(value) => value.clone(),
            None => Value::Nil,
        }
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Value {
        let right = self.evalute(right);
        
        match operator.token_type {
            TokenType::Minus => {
                if let Value::Number(value) = right {
                    Value::Number(-value)
                } else {
                    panic!("Applying '-' operator to a non number.")
                }
            }
            TokenType::Bang => self.is_truthy(&right),
            _ => right
        }
    }

    fn visit_binary(&self, left: &Expr, operation: &Token, right: &Expr) -> Value {
        let left = self.evalute(left);
        let right = self.evalute(right);
        match operation.token_type {
            TokenType::Minus => {
                left - right
            },
            TokenType::Plus => {
                left + right
            },
            TokenType::Slash => {
                left / right
            },
            TokenType::Star => {
                left * right
            },
            TokenType::Greater => {
                if left > right {
                    Value::True
                } else {
                    Value::False
                }
            },
            TokenType::GreaterEqual => {
                if left >= right {
                    Value::True
                } else {
                    Value::False
                }
            },
            TokenType::Less => {
                if left < right {
                    Value::True
                } else {
                    Value::False
                }
            },
            TokenType::LessEqual => {
                if left <= right {
                    Value::True
                } else {
                    Value::False
                }
            },
            TokenType::BangEqual => {
                if left != right {
                    Value::True
                } else {
                    Value::False
                }
            },
            TokenType::EqualEqual => {
                if left == right {
                    Value::True
                } else {
                    Value::False
                }
            },
            _ => panic!("Invalid binary operation: {}", operation.token_type.to_string())
        }
    }

    fn visit_grouping(&self, expression: &Expr) -> Value {
        self.evalute(expression)
    }

    fn visit_ternary(&self, condition: &Expr, then_branch: &Expr, else_branch: &Expr) -> Value {
        if self.is_truthy(&self.evalute(condition)) == Value::True {
            self.evalute(then_branch)
        } else {
            self.evalute(else_branch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlox::test_utils::tests::helper_create_expr_from_string;
    
    #[test]
    fn test_number_equal() {
        let expression = helper_create_expr_from_string("1 == 1");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value, Value::True);
    }

    #[test]
    fn test_nil_equal() {
        let expression = helper_create_expr_from_string("nil == nil");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value, Value::True);
    }

    #[test]
    fn test_bool_equal() {
        let expression = helper_create_expr_from_string("true == true");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value, Value::True);
    }

    #[test]
    fn test_string_equal() {
        let expression = helper_create_expr_from_string("\"hello\" == \"hello\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value, Value::True);
    }

    #[test]
    fn test_number_comparison() {
        let expression = helper_create_expr_from_string("1 < 2");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value, Value::True);
    }
}