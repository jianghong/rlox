use super::token_type::TokenType;

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

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type.to_string(), self.lexeme, self.literal)
    }
}