use crate::tokens::TokenType;
use std::collections::HashMap;
use thiserror::Error;
use std::fmt;


impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::String(s) => write!(f, "{}", s),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in obj {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                    first = false;
                }
                write!(f, "}}")
            },
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                let mut first = true;
                for item in arr {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                    first = false;
                }
                write!(f, "]")
            }
        }
    }
}

impl fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::String(s) => write!(f, "JsonValue::String({:?})", s),
            JsonValue::Number(n) => write!(f, "JsonValue::Number({:?})", n),
            JsonValue::Boolean(b) => write!(f, "JsonValue::Boolean({:?})", b),
            JsonValue::Null => write!(f, "JsonValue::Null"),
            JsonValue::Object(obj) => {
                f.debug_struct("JsonValue::Object")
                 .field("contents", &obj)
                 .finish()
            },
            JsonValue::Array(arr) => {
                f.debug_list()
                 .entries(arr)
                 .finish()
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token")]
    UnexpectedToken,
    #[error("Invalid JSON structure")]
    InvalidStructure,
}

#[derive(Clone)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub struct Parser {
    tokens: Vec<TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<JsonValue, ParserError> {
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParserError> {
        match &self.tokens[self.current] {
            TokenType::LeftBrace => self.parse_object(),
            TokenType::LeftBracket => self.parse_array(),
            TokenType::String(s) => {
                let val = s.clone();
                self.consume();
                Ok(JsonValue::String(val))
            }
            TokenType::Number(n) => {
                let val = *n;
                self.consume();
                Ok(JsonValue::Number(val))
            }
            TokenType::True => {
                self.consume();
                Ok(JsonValue::Boolean(true))
            }
            TokenType::False => {
                self.consume();
                Ok(JsonValue::Boolean(false))
            }
            TokenType::Null => {
                self.consume();
                Ok(JsonValue::Null)
            }
            _ => Err(ParserError::UnexpectedToken),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParserError> {
        self.consume(); // Consume left brace
        let mut object = HashMap::new();

        if let TokenType::RightBrace = self.tokens[self.current] {
            self.consume(); // Consume right brace
            return Ok(JsonValue::Object(object));
        }

        while self.current < self.tokens.len() {
            // Parse key
            let key = match &self.tokens[self.current] {
                TokenType::String(s) => {
                    let k = s.clone();
                    self.consume(); // Consume key
                    k
                }
                _ => return Err(ParserError::UnexpectedToken),
            };

            // Consume colon
            if let TokenType::Colon = self.tokens[self.current] {
                self.consume();
            } else {
                return Err(ParserError::UnexpectedToken);
            }

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);

            // Check for comma or end of object
            match self.tokens[self.current] {
                TokenType::Comma => {
                    self.consume(); // Consume comma
                }
                TokenType::RightBrace => {
                    self.consume(); // Consume right brace
                    return Ok(JsonValue::Object(object));
                }
                _ => return Err(ParserError::UnexpectedToken),
            }
        }

        Err(ParserError::InvalidStructure)
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParserError> {
        self.consume(); // Consume left bracket
        let mut array = Vec::new();

        if let TokenType::RightBracket = self.tokens[self.current] {
            self.consume(); // Consume right bracket
            return Ok(JsonValue::Array(array));
        }

        while self.current < self.tokens.len() {
            let value = self.parse_value()?;
            array.push(value);

            // Check for comma or end of array
            match self.tokens[self.current] {
                TokenType::Comma => {
                    self.consume(); // Consume comma
                }
                TokenType::RightBracket => {
                    self.consume(); // Consume right bracket
                    return Ok(JsonValue::Array(array));
                }
                _ => return Err(ParserError::UnexpectedToken),
            }
        }

        Err(ParserError::InvalidStructure)
    }

    fn consume(&mut self) {
        self.current += 1;
    }
}
