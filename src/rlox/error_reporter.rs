use super::token::Token;
use super::token_type::TokenType;

pub struct ErrorReporter {
    pub had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> ErrorReporter {
        ErrorReporter { had_error: false }
    }

    pub fn error(&mut self, line: u32, message: &String) {
        self.report(line, &"".to_string(), message);
    }

    pub fn token_error(&mut self, token: Token, message: &String) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, &"at end".to_string(), message);
        } else {
            self.report(token.line, &token.lexeme, message);
        }
    }

    fn report(&mut self, line: u32, place: &String, message: &String) {
        println!("[line {line}] Error {place}: {message}");
        self.had_error = true;
    }
}
