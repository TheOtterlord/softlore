pub mod tokenizer;
pub mod constants;
pub mod assignment;

use assignment::Assignment;

#[derive(Debug)]
pub struct CodeLocation {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug)]
pub struct Token {
  pub loc: CodeLocation,
  pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
  Assignment(Assignment),
  Null,
}
