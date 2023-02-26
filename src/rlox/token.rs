use super::token_type::TokenType;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    literal: Option<String>,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: u32) -> Token {
        Token { token_type, lexeme, literal, line }
    }
}