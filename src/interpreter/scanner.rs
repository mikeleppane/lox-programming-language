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
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                false => {
                    if self.is_match('*') {
                        self.scan_comment()?;
                    }
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }
            '0'..='9' => self.number(),
            _ if c.is_ascii_alphabetic() || c == '_' => {
                self.identifier();
            }
            _ => {
                return Err(LoxError::error(
                    self.line,
                    "Unexpected character".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn scan_comment(&mut self) -> Result<(), LoxError> {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();
                    if self.is_match('/') {
                        return Ok(());
                    }
                }
                Some('/') => {
                    self.advance();
                    if self.is_match('*') {
                        self.scan_comment()?;
                    }
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }
                None => {
                    return Err(LoxError::error(
                        self.line,
                        "Unterminated comment".to_string(),
                    ));
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn identifier(&mut self) {
        while Self::is_alpha(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let Some(ttype) = Self::keyword(text.as_str()) else {
            self.add_token(TokenType::Identifier);
            return;
        };
        self.add_token(ttype);
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == Some('.') && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }
        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        let num = value
            .parse()
            .unwrap_or_else(|_| panic!("Could not parse {} to f64", value));
        self.add_token_object(TokenType::Number, Some(Object::Num(num)));
    }

    fn is_digit(ch: Option<char>) -> bool {
        let Some(ch) = ch else {
            return false;
        };
        ch.is_ascii_digit()
    }

    fn is_alpha(ch: Option<char>) -> bool {
        let Some(ch) = ch else {
            return false;
        };
        ch.is_ascii_alphanumeric()
    }

    fn peek_next(&mut self) -> Option<char> {
        self.source.get(self.current + 1).copied()
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
    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    break;
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {}
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LoxError::error(
                self.line,
                "Unterminated string.".to_string(),
            ));
        }
        self.advance();
        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }

    fn keyword(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
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
