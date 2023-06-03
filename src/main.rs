use color_eyre::Result;

use crate::interpreter::lox::Lox;

mod expression;
mod interpreter;
mod tokens;

fn main() -> Result<()> {
    let mut lox = Lox::new();
    lox.main()?;
    Ok(())
}
