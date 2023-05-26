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

    pub fn literal_value<V: Sized + Debug + 'static>(&self) -> Option<&V> {
        if self.token_type == TokenType::String {
            let value = self.literal.as_ref().unwrap().downcast_ref::<V>();
            return value;
        }
        None
    }
}

impl<'a> fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn token_new_creation() {
        let token = Token::new(TokenType::And, "+", None, 1);
        assert_eq!(token.token_type, TokenType::And);
        assert_eq!(token.literal.is_none(), true);
    }

    #[test]
    fn token_new_creation() {
        let token = Token::new(TokenType::And, "+", None, 1);
        assert_eq!(token.token_type, TokenType::And);
        assert_eq!(token.literal.is_none(), true);
    }
}
