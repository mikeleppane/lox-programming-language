use std::fs;
use std::io;
use std::io::Write;

use color_eyre::Result;

use crate::interpreter::error::LoxError;
use crate::interpreter::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

const PRINT_USAGE: &str = "Usage interpreter [script]";

impl Lox {
    pub fn new() -> Self {
        Self { had_error: true }
    }
    pub fn main(&mut self) -> Result<()> {
        let args_passed = std::env::args().count();
        match args_passed {
            n if n > 2 => {
                println!("{}", PRINT_USAGE);
                std::process::exit(64);
            }
            n if n == 2 => {
                let script = std::env::args()
                    .nth(1)
                    .unwrap_or_else(|| panic!("No second argument: {}", PRINT_USAGE));
                self.run_file(&script)?;
            }
            _ => {
                self.run_prompt()?;
            }
        }
        Ok(())
    }

    fn run_file(&self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path)?;
        match self.run(content) {
            Ok(_) => {}
            Err(m) => {
                m.report("");
                std::process::exit(65);
            }
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<()> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        write!(stdout.lock(), "> ")?;
        stdout.flush()?;
        while stdin.read_line(&mut buffer).is_ok() {
            if buffer.is_empty() {
                break;
            }
            if self.run(buffer.trim_end().to_string()).is_ok() {}
            buffer.clear();
            write!(stdout.lock(), "> ")?;
            stdout.flush()?;
            self.had_error = false;
        }
        Ok(())
    }

    fn run(&self, source: String) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        for token in scanner.scan_tokens()? {
            println!("{:?}", token)
        }
        Ok(())
    }
}
