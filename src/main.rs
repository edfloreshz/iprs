use cli::{ParsedArgs, Result};

fn main() -> Result<()> {
    let matches = ParsedArgs::new();
    matches.call();
    Ok(())
}

