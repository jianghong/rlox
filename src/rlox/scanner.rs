use std::collections::HashMap;

use super::error_reporter::ErrorReporter;
use super::token::*;
use super::token_type::*;

pub struct Scanner<'a> {
    source: String,
    pub tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,

    error_reporter: &'a mut ErrorReporter,
    reserved_keywords: HashMap<String, TokenType>,
}

impl Scanner<'_> {
    pub fn new(source: String, error_reporter: &mut ErrorReporter) -> Scanner {
        let reserved_keywords = HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]);
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_reporter: error_reporter,
            reserved_keywords: reserved_keywords,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '?' => self.add_token(TokenType::Question, None),
            ':' => self.add_token(TokenType::Colon, None),
            '!' => {
                let token_type = if self.r#match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.r#match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.r#match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.r#match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.r#match('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.r#match('*') {
                    // C style comment `/*` goes until `*/`
                    while self.peek() != '*' && self.peek_next() != '/' {
                        self.advance();

                        if self.is_at_end() {
                            self.error_reporter
                                .error(self.line, &"Unterminated comment".to_string());
                            break;
                        }
                    }

                    // advance twice to move past `*/`
                    self.advance();
                    self.advance();
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            unexpected => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.error_reporter
                        .error(self.line, &format!("Unexpected character {unexpected}"));
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start as usize..self.current as usize];
        let token_type = self
            .reserved_keywords
            .get(text)
            .unwrap_or(&TokenType::Identifier);
        self.add_token(token_type.clone(), None)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Some(self.source[self.start as usize..self.current as usize].to_string()),
        );
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_reporter
                .error(self.line, &"Unterminated string.".to_string());
        }

        self.advance();

        let value = &self.source[self.start as usize + 1..self.current as usize - 1];
        self.add_token(TokenType::String, Some(value.to_string()));
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap();
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source
            .chars()
            .nth((self.current - 1) as usize)
            .unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }
}
