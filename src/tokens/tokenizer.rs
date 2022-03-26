use crate::error::{CompilerError, ErrorType};
use super::Token;

pub struct Tokenizer {
  pub code: Vec<char>,
  pub pos: usize,
  pub x: usize,
  pub y: u16
}

impl Tokenizer {
  pub fn tokenize(code: &str) -> Result<Vec<Token>, CompilerError> {
    let mut tokenizer = Tokenizer {
      code: code.chars().collect(),
      pos: 0,
      x: 0,
      y: 0
    };

    let mut tokens: Vec<Token> = Vec::new();

    while tokenizer.pos < tokenizer.code.len() {
      let expr = tokenizer.parse_expression();

      match expr {
        Ok(token) => tokens.push(token),
        Err(err) => return Err(err)
      }
    }

    return Ok(tokens);
  }

  pub fn parse_expression(&mut self) -> Result<Token, CompilerError> {
    return Err(CompilerError::new(
      ErrorType::InternalError,
      "Not implemented".to_string(),
      self.x,
      self.y
    ))
  }
}
