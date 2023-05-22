use std::fmt;

use crate::tokens::token_type::TokenType;

struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    line: u64,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str) -> Self {
        Self {
            token_type,
            lexeme,
            line: 0,
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}
