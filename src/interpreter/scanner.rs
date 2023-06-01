use color_eyre::Result;

use crate::interpreter::error::LoxError;
use crate::tokens::token::{Object, Token};
use crate::tokens::token_type::TokenType;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("");
                    had_error = Some(e);
                }
            }
        }
        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => match self.is_match('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang),
            },
            '=' => match self.is_match('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },
            '<' => match self.is_match('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less),
            },
            '>' => match self.is_match('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater),
            },
            '/' => match self.is_match('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::Slash),
            },
            _ => {
                return Err(LoxError::error(
                    self.line,
                    "Unexpected character".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .get(self.current)
            .unwrap_or_else(|| panic!("No character in source at position {}", self.current));
        self.current += 1;
        *c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_object(token_type, None);
    }

    fn add_token_object(&mut self, token_type: TokenType, literal: Option<Object>) {
        let y = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, y, literal, self.line));
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current)
            .unwrap_or_else(|| panic!("No character at position {}", self.current))
    }
    //
    // fn string(&mut self) {
    //     while self.peek() != '"' && !self.is_at_end() {
    //         if self.peek() == '\n' {
    //             self.line += 1;
    //         }
    //         self.advance();
    //     }
    //     if self.is_at_end() {
    //         report(self.line, "", "Unterminated string");
    //         return;
    //     }
    //     self.advance();
    //
    //     let value = self.source[self.start + 1..self.current - 1].to_string();
    //     self.add_token_object(TokenType::String, Some(Box::new(value)))
    // }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_scan_comments() {
        let source = "//\n";
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan_tokens().expect("");
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn should_scan_groups() {
        let source = "((";
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 3);
        assert_eq!(scanner.tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[1].token_type, TokenType::LeftParen);
    }

    #[test]
    fn should_scan_operators() {
        let source = "!*+-/=<> <= ==";
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 11);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Bang);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Star);
    }

    // #[test]
    // fn should_scan_string_literals() {
    //     let source = "\"this is a multistring input\"";
    //     let mut scanner = Scanner::new(source.to_string());
    //     scanner.scan_tokens();
    //     let value = scanner.tokens[0].literal_value::<String>();
    //     if let Some(v) = value {
    //         assert_eq!(v, "this is a multistring input")
    //     } else {
    //         panic!("Should contain a value")
    //     }
    // }
    //
    // #[test]
    // fn scan_tokens() {
    //     let source = vec![
    //         "(", ")", "{", "}", ",", ".", "<", ">", "!", "!=", "<=", ">=",
    //     ];
    //     let source_as_str = source.join("");
    //     let mut scanner = Scanner::new(source_as_str.as_str());
    //     scanner.scan_tokens();
    //     assert_eq!(scanner.tokens.len(), 13);
    //     for (index, token) in scanner.tokens.iter().enumerate() {
    //         dbg!(token);
    //         if index < source.len() {
    //             assert_eq!(token.lexeme, source[index]);
    //         }
    //     }
    // }
}
