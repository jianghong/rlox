use super::error_reporter;
use super::token::*;
use super::token_type::*;
use super::error_reporter::ErrorReporter;

pub struct Scanner<'a> {
    source: String,
    pub tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,

    error_reporter: &'a mut ErrorReporter,
}

impl Scanner<'_> {
    pub fn new(source: String, error_reporter: &mut ErrorReporter) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_reporter: error_reporter
        }
    }
    
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), "".to_string(), 0));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }
    
    fn scan_token(&mut self) {
        let c = self.advance();

        println!("current: {} c: {}", self.current, c);
        match c {
            '(' => self.add_token(TokenType::LeftParen, "".to_string()),
            ')' => self.add_token(TokenType::RightParen, "".to_string()),
            '{' => self.add_token(TokenType::LeftBrace, "".to_string()),
            '}' => self.add_token(TokenType::RightBrace, "".to_string()),
            ',' => self.add_token(TokenType::Comma, "".to_string()),
            '.' => self.add_token(TokenType::Dot, "".to_string()),
            '-' => self.add_token(TokenType::Minus, "".to_string()),
            '+' => self.add_token(TokenType::Plus, "".to_string()),
            ';' => self.add_token(TokenType::Semicolon, "".to_string()),
            '*' => self.add_token(TokenType::Star, "".to_string()),
            '!' => {
                let token_type = if self.r#match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, "".to_string());
            },
            '=' => {
                let token_type = if self.r#match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, "".to_string());
            },
            '<' => {
                let token_type = if self.r#match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, "".to_string());
            },
            '>' => {
                let token_type = if self.r#match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, "".to_string());
            },
            '/' => {
                if self.r#match('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, "".to_string());
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            _ => self.error_reporter.error(self.line, &"Unexpected character.".to_string()),
        }
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }

        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false
        }

        self.current += 1;
        return true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }

        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        // get substring in source from start to current
        let lexeme = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(
            Token::new(
                token_type,
                lexeme.to_string(),
                literal,
                self.line
            )
        );
    }
}