use color_eyre::Result;

use crate::lox::lox::Lox;

mod lox;

fn main() -> Result<()> {
    let lox = Lox::new();
    lox.main()?;
    Ok(())
}
