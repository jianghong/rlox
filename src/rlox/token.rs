use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Token {
        Token { token_type, lexeme, literal, line }
    }
}