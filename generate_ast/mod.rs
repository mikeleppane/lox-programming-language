use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &Path) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        &[
            "Binary: Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping: Box<Expr> expression".to_string(),
            "Literal: Object value".to_string(),
            "Unary: Token operator, Box<Expr> right".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &Path, base_name: &str, types: &[String]) -> io::Result<()> {
    let mut filename = base_name.to_lowercase();
    filename.push_str(".rs");
    let path = output_dir.join(filename);
    let mut file = File::create(path)?;
    let mut tree_types: Vec<TreeType> = Vec::new();

    writeln!(file, "use color_eyre::Result;")?;
    writeln!(file, "use crate::interpreter::error::LoxError;")?;
    writeln!(file, "use crate::tokens::token::{{Object, Token}};")?;
    writeln!(file)?;
    for ttype in types {
        let (base_class_name, args) = ttype.split_once(':').unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split = args.split(',');
        let mut fields: Vec<String> = Vec::new();
        for arg in arg_split {
            let (var_type, name) = arg.trim().split_once(' ').unwrap();
            fields.push(format!("{}: {}", name, var_type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        })
    }

    writeln!(file, "pub enum {} {{", base_name)?;
    for t in &tree_types {
        writeln!(file, "\t{}({}),", t.base_class_name, t.class_name)?;
    }
    writeln!(file, "}}\n")?;

    for t in &tree_types {
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for f in &t.fields {
            writeln!(file, "    {},", f)
                .unwrap_or_else(|_| panic!("cannot write to file: {:?}", file.metadata()));
        }
        writeln!(file, "}}\n\n")?;
    }

    writeln!(file, "pub trait ExprVisitor<T> {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError>;",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
        )?;
    }
    writeln!(file, "}}\n\n")?;

    for t in &tree_types {
        writeln!(file, "impl {} {{", t.class_name)?;
        writeln!(
            file,
            "    fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
            base_name
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
    }

    Ok(())
}
