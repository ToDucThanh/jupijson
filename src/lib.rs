mod tokens;
mod scanner;
mod parser;

pub use parser::JsonValue;
use scanner::Scanner;
use parser::Parser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonParseError {
    #[error("Scanning error")]
    ScanningError(#[from] scanner::ScannerError),
    #[error("Parsing error")]
    ParsingError(#[from] parser::ParserError),
}

pub fn loads(json_str: &str) -> Result<JsonValue, JsonParseError> {
    let mut scanner = Scanner::new(json_str);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let result = parser.parse()?;
    Ok(result)
}