use std::any::Any;
use std::fmt;
use std::fmt::Debug;

use crate::tokens::token_type::TokenType;

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
    pub literal: Option<Box<dyn Any>>,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<Box<dyn Any>>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}

impl<'a> fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_new_creation() {
        let token = Token::new(TokenType::And, "+", None, 1);
        println!("{token}")
    }
}
