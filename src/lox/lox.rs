use std::fs;
use std::io;
use std::io::Write;

use color_eyre::Result;

pub struct Lox {}

const PRINT_USAGE: &str = "Usage lox [script]";

impl Lox {
    pub fn main(&self) -> Result<()> {
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

    fn run_prompt(&self) -> Result<()> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        write!(stdout.lock(), "> ")?;
        stdout.flush()?;
        while stdin.read_line(&mut buffer).is_ok() {
            let trimmed = buffer.trim_end();
            println!("You typed: [{trimmed}]");
            buffer.clear();
            write!(stdout.lock(), ">")?;
            stdout.flush()?;
        }
        Ok(())
    }

    fn run(&self, source: &str) -> Result<()> {
        Ok(())
    }
}
