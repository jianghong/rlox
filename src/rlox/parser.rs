use anyhow::{Error, Result, anyhow};


use super::token::Token;
use super::token_type::TokenType;
use super::expr::*;
use super::error_reporter::ErrorReporter;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,

    error_reporter: &'a mut ErrorReporter,
}


impl Parser<'_> {
    pub fn new(tokens: Vec<Token>, error_reporter: &mut ErrorReporter ) -> Parser {
        Parser { tokens, current: 0, error_reporter: error_reporter }
    }
    
    pub fn parse<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>> {
        self.expression()
    }

    fn expression<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>> {
        self.equality()
    }

    fn equality<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>> {
        let mut expr = self.comparison()?;

        while self.r#match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn comparison<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>> {
        let mut expr = self.term()?;

        while self.r#match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn term<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>> {
        let mut expr = self.factor()?;

        while self.r#match(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn factor<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>> {
        let mut expr = self.unary()?;

        while self.r#match(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn unary<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>> {
        if self.r#match(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Box::new(Unary { operator, right }));
        }

        self.primary()
    }

    fn primary<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>> {
        if self.r#match(vec![TokenType::False]) {
            return Ok(Box::new(Literal {
                value: Some("false".to_string()),
            }));
        }

        if self.r#match(vec![TokenType::True]) {
            return Ok(Box::new(Literal {
                value: Some("true".to_string()),
            }));
        }

        if self.r#match(vec![TokenType::Nil]) {
            return Ok(Box::new(Literal {
                value: Some("nil".to_string()),
            }));
        }

        if self.r#match(vec![TokenType::Number, TokenType::String]) {
            return Ok(Box::new(Literal {
                value: Some(self.previous().lexeme.clone()),
            }));
        }

        if self.r#match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Box::new(Grouping { expression: expr }));
        }

        Err(anyhow!("Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<()> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            let message = message.to_string();
            self.error_reporter.token_error(self.peek().clone(), &message);
            Err(anyhow!(message))
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn r#match(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
}