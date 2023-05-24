use std::any::Any;

use crate::interpreter::errors::report;
use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }
    fn scan_token(&mut self) {
        let Some(c) = self.advance() else {
            println!("No more tokens available");
            return;
        };
        match c {
            '(' => self.add_plain_token(TokenType::LeftParen),
            ')' => self.add_plain_token(TokenType::RightParen),
            '{' => self.add_plain_token(TokenType::LeftBrace),
            '}' => self.add_plain_token(TokenType::RightBrace),
            ',' => self.add_plain_token(TokenType::Comma),
            '.' => self.add_plain_token(TokenType::Dot),
            '-' => self.add_plain_token(TokenType::Minus),
            '+' => self.add_plain_token(TokenType::Plus),
            ';' => self.add_plain_token(TokenType::Semicolon),
            '*' => self.add_plain_token(TokenType::Star),
            '!' => match self.match_token('=') {
                true => self.add_plain_token(TokenType::BangEqual),
                false => self.add_plain_token(TokenType::Bang),
            },
            '=' => match self.match_token('=') {
                true => self.add_plain_token(TokenType::EqualEqual),
                false => self.add_plain_token(TokenType::Equal),
            },
            '<' => match self.match_token('=') {
                true => self.add_plain_token(TokenType::LessEqual),
                false => self.add_plain_token(TokenType::Less),
            },
            '>' => match self.match_token('=') {
                true => self.add_plain_token(TokenType::GreaterEqual),
                false => self.add_plain_token(TokenType::Greater),
            },
            _ => report(self.line, "", "Unexpected character."),
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(c) = self.source.chars().nth(self.current) {
            if c != expected {
                return false;
            }
        }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current)
    }

    fn add_plain_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let y = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, y, None, self.line));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scan_tokens() {
        let source = "(){},.";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        println!("{}", scanner.tokens.len());
        for token in scanner.tokens {
            println!("{:?}", token);
        }
    }
}
