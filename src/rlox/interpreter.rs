use anyhow::{anyhow, Result};

use crate::rlox::error_reporter::ErrorReporter;
use crate::rlox::expr::*;
use crate::rlox::token::Token;
use crate::rlox::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&self, expression: &Expr, error_reporter: &mut ErrorReporter) {
        let result = self.evalute(expression);
        match result {
            Ok(value) => println!("{}", self.stringify(&value)),
            Err(error) => error_reporter.error(0, &error.to_string()),
        }
    }

    fn evalute(&self, expression: &Expr) -> Result<Value> {
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

impl Visitor<Result<Value>> for Interpreter {
    fn visit_literal(&self, value: &Option<Value>) -> Result<Value> {
        let value = match value {
            Some(value) => value.clone(),
            None => Value::Nil,
        };

        Ok(value)
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<Value> {
        let right = self.evalute(right)?;

        match operator.token_type {
            TokenType::Minus => {
                if let Value::Number(value) = right {
                    Ok(Value::Number(-value))
                } else {
                    Err(anyhow!("Applying '-' operator to a non number."))
                }
            }
            TokenType::Bang => Ok(self.is_truthy(&right)),
            _ => Ok(right),
        }
    }

    fn visit_binary(&self, left: &Expr, operation: &Token, right: &Expr) -> Result<Value> {
        let left = self.evalute(left)?;
        let right = self.evalute(right)?;
        match operation.token_type {
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::Greater => {
                if !left.is_number() || !right.is_number() {
                    return Err(anyhow!("Applying '>' operator to a non number."));
                }
                let value = if left > right {
                    Value::True
                } else {
                    Value::False
                };
                Ok(value)
            }
            TokenType::GreaterEqual => {
                if !left.is_number() || !right.is_number() {
                    return Err(anyhow!("Applying '>' operator to a non number."));
                }
                let value = if left >= right {
                    Value::True
                } else {
                    Value::False
                };
                Ok(value)
            }
            TokenType::Less => {
                if !left.is_number() || !right.is_number() {
                    return Err(anyhow!("Applying '>' operator to a non number."));
                }
                let value = if left < right {
                    Value::True
                } else {
                    Value::False
                };
                Ok(value)
            }
            TokenType::LessEqual => {
                if !left.is_number() || !right.is_number() {
                    return Err(anyhow!("Applying '>' operator to a non number."));
                }
                let value = if left <= right {
                    Value::True
                } else {
                    Value::False
                };
                Ok(value)
            }
            TokenType::BangEqual => {
                let value = if left != right {
                    Value::True
                } else {
                    Value::False
                };
                Ok(value)
            }
            TokenType::EqualEqual => {
                let value = if left == right {
                    Value::True
                } else {
                    Value::False
                };
                Ok(value)
            }
            _ => Err(anyhow!(
                "Invalid binary operation: {}",
                operation.token_type.to_string()
            )),
        }
    }

    fn visit_grouping(&self, expression: &Expr) -> Result<Value> {
        self.evalute(expression)
    }

    fn visit_ternary(
        &self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<Value> {
        if self.is_truthy(&self.evalute(condition)?) == Value::True {
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
        assert_eq!(value.unwrap(), Value::True);
    }

    #[test]
    fn test_nil_equal() {
        let expression = helper_create_expr_from_string("nil == nil");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value.unwrap(), Value::True);
    }

    #[test]
    fn test_bool_equal() {
        let expression = helper_create_expr_from_string("true == true");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value.unwrap(), Value::True);
    }

    #[test]
    fn test_string_equal() {
        let expression = helper_create_expr_from_string("\"hello\" == \"hello\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value.unwrap(), Value::True);
    }

    #[test]
    fn test_number_comparison() {
        let expression = helper_create_expr_from_string("1 < 2");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value.unwrap(), Value::True);
    }

    #[test]
    fn test_unary_minus_on_non_number() {
        let expression = helper_create_expr_from_string("-\"hello\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert!(value.is_err());
    }

    #[test]
    fn test_add_number_with_non_number() {
        let expression = helper_create_expr_from_string("1 + \"world\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value.unwrap(), Value::String("1world".to_string()));
    }

    #[test]
    fn test_sub_number_with_non_number() {
        let expression = helper_create_expr_from_string("1 - \"world\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert!(value.is_err());
    }

    #[test]
    fn test_mul_number_with_non_number() {
        let expression = helper_create_expr_from_string("1 * \"world\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert!(value.is_err());
    }

    #[test]
    fn test_div_number_with_non_number() {
        let expression = helper_create_expr_from_string("1 / \"world\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert!(value.is_err());
    }

    #[test]
    fn test_compare_number_with_non_number() {
        let expression = helper_create_expr_from_string("1 < \"world\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert!(value.is_err());
    }

    #[test]
    fn test_number_equal_non_number() {
        let expression = helper_create_expr_from_string("1 == \"world\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value.unwrap(), Value::False);
    }

    #[test]
    fn test_add_string_with_non_string() {
        let expression = helper_create_expr_from_string("\"hello\" + 1");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value.unwrap(), Value::String("hello1".to_string()));
    }

    #[test]
    fn test_add_non_string_with_string() {
        let expression = helper_create_expr_from_string("true + \"hello\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(value.unwrap(), Value::String("truehello".to_string()));
    }

    #[test]
    fn test_adding_string_with_non_string_n_times() {
        let expression = helper_create_expr_from_string("true + \"hello\" + 1 + nil + \"world\"");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert_eq!(
            value.unwrap(),
            Value::String("truehello1nilworld".to_string())
        );
    }

    #[test]
    fn test_divide_by_zero_reports_runtime_error() {
        let expression = helper_create_expr_from_string("1 / 0");
        let interpreter = Interpreter::new();
        let value = interpreter.evalute(&expression);
        assert!(value.is_err());
    }
}
