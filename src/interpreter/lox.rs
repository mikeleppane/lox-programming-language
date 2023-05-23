use std::fs;
use std::io;
use std::io::Write;

use color_eyre::Result;

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
        self.run(&content)?;
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<()> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        write!(stdout.lock(), "> ")?;
        stdout.flush()?;
        while stdin.read_line(&mut buffer).is_ok() {
            self.run(buffer.trim_end())?;
            buffer.clear();
            write!(stdout.lock(), ">")?;
            stdout.flush()?;
            self.had_error = false;
        }
        Ok(())
    }

    fn run(&self, _source: &str) -> Result<()> {
        Ok(())
    }

    pub fn error(&mut self, line: u64, message: &str) {
        self.report(line, "", message);
        self.had_error = true;
    }

    fn report(&self, line: u64, place: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, place, message);
    }
}
