#[derive(Debug)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> Self {
        Self { line, message }
    }
    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error {}: {}", self.line, loc, self.message);
    }
}
