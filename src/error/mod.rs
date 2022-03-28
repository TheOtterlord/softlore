#[derive(Debug)]
pub struct CompilerError {
  pub error_type: ErrorType,
  pub message: String,
  pub line: usize,
  pub column: usize,
}

impl CompilerError {
  pub fn new(error_type: ErrorType, message: String, line: usize, column: usize) -> CompilerError {
    return CompilerError {
      error_type: error_type,
      message: message,
      line: line,
      column: column,
    };
  }
}

impl CompilerError {
  pub fn to_string(&self, code: &str) -> String {
    let lines: Vec<&str> = code.split("\n").collect();

    let number_spaces = (self.line + 1).to_string().len();

    let before: String;
    if self.line > 1 {
      before = format!("{}{} | {}", " ".repeat(number_spaces - (self.line - 1).to_string().len()), self.line - 1, lines.get(self.line - 2).unwrap_or(&""));
    } else {
      before = String::new();
    }
    let current = format!("{}{} | {}", " ".repeat(number_spaces - (self.line).to_string().len()), self.line, lines[self.line - 1]);
    let after = format!("{} | {}", self.line + 1, lines.get(self.line).unwrap_or(&""));

    let pointer = format!(
      "{} | {}{}",
      " ".repeat(number_spaces),
      " ".repeat(self.column),
      "^"
    );

    return format!(
      "+++ {}: {} +++\n{}\n{}\n{}\n{}",
      self.error_type.to_string(),
      self.message,
      before,
      current,
      pointer,
      after
    );
  }
}

#[derive(Debug)]
pub enum ErrorType {
  SyntaxError,
  InternalError,
}

impl ToString for ErrorType {
  fn to_string(&self) -> String {
    match self {
      ErrorType::SyntaxError => "SyntaxError".to_string(),
      ErrorType::InternalError => "InternalError".to_string(),
    }
  }
}
