pub mod tokens;
pub mod error;

use tokens::tokenizer::Tokenizer;

fn main() {
  match Tokenizer::tokenize("") {
    Ok(tokens) => println!("{:?}", tokens),
    Err(err) => println!("{:?}", err)
  }
}
