use anyhow::{anyhow, Result};

use super::{
    error_reporter::ErrorReporter,
    expr::{Expr, Value},
    token::Token,
    token_type::TokenType,
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,

    error_reporter: &'a mut ErrorReporter,
}

impl Parser<'_> {
    pub fn new(tokens: Vec<Token>, error_reporter: &mut ErrorReporter) -> Parser {
        Parser {
            tokens,
            current: 0,
            error_reporter: error_reporter,
        }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.r#match(vec![TokenType::Question]) {
            let then_branch = self.expression()?;
            self.consume(TokenType::Colon, "Expect ':' after then branch.")?;
            let else_branch = self.expression()?;
            expr = Expr::Ternary {
                condition: Box::new(expr),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            };
        }

        while self.r#match(vec![TokenType::Comma]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.r#match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.r#match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.r#match(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.r#match(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.r#match(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.r#match(vec![TokenType::False]) {
            return Ok(Expr::Literal(Some(Value::False)));
        }

        if self.r#match(vec![TokenType::True]) {
            return Ok(Expr::Literal(Some(Value::True)));
        }

        if self.r#match(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Some(Value::Nil)));
        }

        if self.r#match(vec![TokenType::Number]) {
            return Ok(Expr::Literal(Some(Value::to_number(
                &self.previous().lexeme,
            ))));
        }

        if self.r#match(vec![TokenType::String]) {
            return Ok(Expr::Literal(Some(Value::to_string(
                &self.previous().lexeme[1..self.previous().lexeme.len() - 1],
            ))));
        }

        if self.r#match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(anyhow!("Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<()> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            let message = message.to_string();
            self.error_reporter
                .token_error(self.peek().clone(), &message);
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
