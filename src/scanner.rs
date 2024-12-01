use crate::tokens::TokenType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Invalid token encountered")]
    InvalidToken,
}

pub struct Scanner {
    source: String,
    tokens: Vec<TokenType>,
    current: usize,
    start: usize,
}

impl Scanner {
    pub fn new(json_str: &str) -> Self {
        Scanner {
            source: json_str.to_string(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ch
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<TokenType>, ScannerError> {
        while !self.is_at_end() {
            // Skip whitespace
            while let Some(c) = self.peek() {
                if !c.is_whitespace() {
                    break;
                }
                self.advance();
            }

            if self.is_at_end() {
                break;
            }

            self.start = self.current;
            self.scan_token()?;
        }
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        let c = self.advance();
        match c {
            '{' => self.tokens.push(TokenType::LeftBrace),
            '}' => self.tokens.push(TokenType::RightBrace),
            '[' => self.tokens.push(TokenType::LeftBracket),
            ']' => self.tokens.push(TokenType::RightBracket),
            ':' => self.tokens.push(TokenType::Colon),
            ',' => self.tokens.push(TokenType::Comma),
            '"' => self.scan_string()?,
            c if Self::is_digit(c) || c == '-' => self.scan_number(),
            c if c.is_alphabetic() => self.scan_keyword(),
            _ => return Err(ScannerError::InvalidToken),
        }
        Ok(())
    }

    fn scan_string(&mut self) -> Result<(), ScannerError> {
        let mut string_value = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // Consume closing quote
                break;
            }
            string_value.push(self.advance());
        }
        self.tokens.push(TokenType::String(string_value));
        Ok(())
    }

    fn scan_number(&mut self) {
        let start = self.current - 1; // Go back to include the first digit/minus sign
        let mut end = start;

        // Allow for optional minus sign, digits, optional decimal point, and optional exponent
        while let Some(ch) = self.source.chars().nth(end) {
            if ch.is_ascii_digit() || 
               ch == '.' || 
               ch == 'e' || 
               ch == 'E' || 
               ch == '-' || 
               ch == '+' {
                end += 1;
            } else {
                break;
            }
        }

        let number_str = &self.source[start..end];
        if let Ok(number) = number_str.parse::<f64>() {
            self.current = end;
            self.tokens.push(TokenType::Number(number));
        }
    }

    fn scan_keyword(&mut self) {
        let start = self.current - 1;
        
        // Find the end of the keyword
        while let Some(ch) = self.peek() {
            if !ch.is_alphabetic() {
                break;
            }
            self.advance();
        }

        let keyword = &self.source[start..self.current];
        match keyword {
            "true" => self.tokens.push(TokenType::True),
            "false" => self.tokens.push(TokenType::False),
            "null" => self.tokens.push(TokenType::Null),
            _ => {} // Ignore other alphabetic sequences
        }
    }
}