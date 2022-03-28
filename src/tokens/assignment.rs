use crate::error::{CompilerError, ErrorType};
use crate::Tokenizer;
use crate::tokens::{Expression, Token};

#[derive(Debug)]
pub struct Assignment {
  pub name: String,
  pub value: Box<Expression>,
}

pub fn parse_assignment(t: &mut Tokenizer, keyword: String) -> Result<Token, CompilerError> {
  return Err(CompilerError::new(
    ErrorType::InternalError,
    "Assignment not implemented yet".to_string(),
    t.y,
    t.x,
  ));
}
