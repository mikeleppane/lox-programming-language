use std::any::Any;
use std::fmt;

use crate::tokens::token_type::TokenType;

struct Token<'a, T: Any + 'static> {
    token_type: TokenType,
    lexeme: &'a str,
    line: u64,
    literal: T,
}

impl<'a, T> Token<'a, T> {
    pub fn new(token_type: TokenType, lexeme: &'a str, literal: T, line: u64) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}

impl<T: Any + 'static> fmt::Display for Token<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_new_creation() {
        let token = Token::new(TokenType::AND, "+", "", 1);
    }
}
