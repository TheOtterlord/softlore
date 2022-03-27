pub mod tokens;
pub mod error;

use tokens::tokenizer::Tokenizer;

pub type SyntaxOption = Vec<Vec<String>>;

pub struct SyntaxOptions {
  pub assignment: SyntaxOption
}

pub struct CompilerOptions {
  pub syntax: SyntaxOptions
}

fn main() {
  match Tokenizer::tokenize("set") {
    Ok(tokens) => println!("{:?}", tokens),
    Err(err) => println!("{:?}", err)
  }
}
