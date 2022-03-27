pub mod tokenizer;
pub mod constants;
pub mod assignment;

use tokenizer::Snapshot;
use assignment::Assignment;

#[derive(Debug)]
pub struct CodeLocation {
  pub line: u16,
  pub column: usize,
}

impl From <Snapshot> for CodeLocation {
  fn from(snapshot: Snapshot) -> Self {
    CodeLocation {
      line: snapshot.y,
      column: snapshot.x
    }
  }
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
