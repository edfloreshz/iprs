use cli::ParsedArgs;

fn main() {
    let matches = ParsedArgs::new();
    matches.call();
}

