use color_eyre::Result;

use crate::lox::lox::Lox;

mod lox;
mod tokens;

fn main() -> Result<()> {
    let mut lox = Lox::new();
    lox.main()?;
    Ok(())
}
