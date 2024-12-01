#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftBrace => write!(f, "LEFT_BRACE '{{'"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE '}}'"),
            TokenType::LeftBracket => write!(f, "LEFT_BRACKET '['"),
            TokenType::RightBracket => write!(f, "RIGHT_BRACKET ']'"),
            TokenType::Colon => write!(f, "COLON ':'"),
            TokenType::Comma => write!(f, "COMMA ','"),
            TokenType::String(s) => write!(f, "STRING '{}'", s),
            TokenType::Number(n) => write!(f, "NUMBER {}", n),
            TokenType::True => write!(f, "TRUE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Null => write!(f, "NULL"),
        }
    }
}