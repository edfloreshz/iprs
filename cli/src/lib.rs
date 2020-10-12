pub mod io;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
