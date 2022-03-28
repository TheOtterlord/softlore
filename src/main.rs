pub mod tokens;
pub mod error;

use tokens::tokenizer::Tokenizer;
use serde::Deserialize;

pub type SyntaxOption = Vec<Vec<String>>;

#[derive(Deserialize)]
pub struct SyntaxOptions {
  pub assignment: SyntaxOption,
  pub null: String,
}

#[derive(Deserialize)]
pub struct CompilerOptions {
  pub syntax: SyntaxOptions
}

fn main() {
  // TODO: combine with custom options
  let options: CompilerOptions = serde_json::from_str(include_str!("../config.json")).unwrap();

  let code = "\n\n\n\n\n\n\nnull\nset name to \"Otterlord\"\nnull\nnull";

  match Tokenizer::tokenize(code) {
    Ok(tokens) => println!("{:?}", tokens),
    Err(err) => println!("{}", err.to_string(code))
  }
}
