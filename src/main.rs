use cli::io::utils;
use std::{env, process};

fn main() {
    let config = utils::Config::new(env::args()).unwrap_or_else(|e| {
        println!("An error occurred: {}", e);
        process::exit(1)
    });
    if let Err(e) = utils::run(config) {
        println!("{}", e.to_string());
        process::exit(1)
    }
}
