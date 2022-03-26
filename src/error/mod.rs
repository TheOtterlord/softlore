#[derive(Debug)]
pub struct CompilerError {
  pub error_type: ErrorType,
  pub message: String,
  pub line: usize,
  pub column: u16,
}

impl CompilerError {
  pub fn new(error_type: ErrorType, message: String, line: usize, column: u16) -> CompilerError {
    return CompilerError {
      error_type: error_type,
      message: message,
      line: line,
      column: column,
    };
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
