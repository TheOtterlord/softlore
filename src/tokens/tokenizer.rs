use crate::tokens::assignment::parse_assignment;
use crate::SyntaxOptions;
use crate::SyntaxOption;
use crate::CompilerOptions;
use crate::tokens::constants::WORDBREAK;
use crate::error::{CompilerError, ErrorType};
use super::Token;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Builder {
  Assignment
}

pub struct Tokenizer {
  pub code: Vec<char>,
  options: CompilerOptions,
  start_keywords: Vec<String>,
  keyword_to_builder: HashMap<String, Builder>,
  pub pos: usize,
  pub x: usize,
  pub y: u16
}

impl Tokenizer {
  pub fn tokenize(code: &str) -> Result<Vec<Token>, CompilerError> {
    let options = CompilerOptions {
      syntax: SyntaxOptions {
        assignment: vec![vec!["set".into(), ":name".into(), "to".into(), ":value".into()]]
      }
    };

    let mut start_keywords = Vec::new();
    let mut keyword_to_builder = HashMap::new();

    let err = Tokenizer::add_keywords(&mut start_keywords, &mut keyword_to_builder, &options.syntax.assignment, Builder::Assignment);

    if err.is_err() {
      return Err(err.unwrap_err());
    }

    let mut tokenizer = Tokenizer {
      options,
      start_keywords,
      keyword_to_builder,
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
    let keyword = self.try_match(&self.start_keywords.clone());

    if keyword.is_some() {
      let keyword = keyword.unwrap();
      let builder = self.keyword_to_builder.get(&keyword).unwrap();
      return match builder {
        Builder::Assignment => parse_assignment(self, keyword)
      };
    }

    return Err(CompilerError::new(
      ErrorType::InternalError,
      "Not implemented".to_string(),
      self.x,
      self.y
    ))
  }

  pub fn try_match(&mut self, keywords: &Vec<String>) -> Option<String> {
    let snapshot = self.snapshot();

    let mut valid_keywords = keywords.clone();

    let mut c = self.next_except(WORDBREAK);
    if c.is_none() {
      self.restore_snapshot(&snapshot);
      return None;
    }

    let mut word = String::new();

    loop {
      if c.is_none() || WORDBREAK.contains(c.unwrap()) {
        if valid_keywords.contains(&word) {
          return Some(word)
        }
        break;
      }

      valid_keywords.retain(|x| x.starts_with(&word));
      if valid_keywords.len() == 0 {
        break;
      }

      word += &c.unwrap().to_string();
      c = self.next();
    }

    None
  }

  pub fn next(&mut self) -> Option<char> {
    if self.y != 0 {
      self.pos += 1;
      self.x += 1;
    } else {
      self.y = 1;
    }

    return if let Some(c) = self.code.get(self.pos) {
      if c == &'\n' {
        self.x = 0;
        self.y += 1;
      }
      Some(*c)
    } else {
      None
    };
  }

  pub fn next_except(&mut self, except: &str) -> Option<char> {
    let mut next = self.next();
    while next.is_some() && except.contains(next.unwrap()) {
      next = self.next();
    }
    return next;
  }

  fn add_keywords(
    start_keywords: &mut Vec<String>,
    keyword_to_builder: &mut HashMap<String, Builder>,
    option: &SyntaxOption,
    builder: Builder,
  ) -> Result<(), CompilerError> {
    for pattern in option {
      start_keywords.push(pattern[0].clone());
      let exists = keyword_to_builder.insert(pattern[0].clone(), builder.clone());
      if exists.is_some() {
        return Err(CompilerError::new(
          ErrorType::SyntaxError,
          format!("Duplicate keyword: {}", pattern[0]),
          0,
          0
        ));
      }
    }
    Ok(())
  }

  pub fn snapshot(&mut self) -> Snapshot {
    Snapshot {
      pos: self.pos,
      x: self.x,
      y: self.y
    }
  }

  pub fn restore_snapshot(&mut self, snapshot: &Snapshot) {
    self.pos = snapshot.pos;
    self.x = snapshot.x;
    self.y = snapshot.y;
  }
}

pub struct Snapshot {
  pub pos: usize,
  pub x: usize,
  pub y: u16
}
