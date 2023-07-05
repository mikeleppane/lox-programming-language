use std::io;
use std::path::Path;

use generate_ast::*;

mod generate_ast;

fn main() -> io::Result<()> {
    generate_ast(Path::new("src"))?;
    Ok(())
}
