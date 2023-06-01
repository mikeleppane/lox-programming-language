use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::tokens::token_type::TokenType;

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Object>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Object>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            literal,
        }
    }

    pub fn eof(line: usize) -> Self {
        Self {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line,
            literal: None,
        }
    }

    // pub fn literal_value<V: Sized + Debug + 'static>(&self) -> Option<&V> {
    //     if self.token_type == TokenType::String {
    //         let value = self.literal.as_ref().unwrap().downcast_ref::<V>();
    //         return value;
    //     }
    //     None
    // }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn token_new_creation() {
        let token = Token::new(TokenType::And, "+".to_string(), None, 1);
        assert_eq!(token.token_type, TokenType::And);
        assert_eq!(token.literal.is_none(), true);
    }
}
