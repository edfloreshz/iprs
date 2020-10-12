use crate::errors::custom::CustomError;
use dirs::home_dir;
use std::fs::File;
use std::path::Path;
use std::{error, fs};

pub fn initialize(force: bool) -> Result<(), Box<dyn error::Error>> {
    if !force {
        if Path::new(&make_path("")?).exists() {
            return Err(CustomError::new(
                "Configuration already exists, try using -f to \
            reinitialize, this will delete any previous configuration and files in your account.",
            ));
        }
    }
    match make_config() {
        Ok(..) => Ok(()),
        Err(e) => Err(e),
    }
}

fn make_config() -> Result<(), Box<dyn error::Error>> {
    let config_paths = vec![make_path("config")?, make_path("database")?];
    let file_paths = vec![make_path("database/files.db")?];
    for path in config_paths {
        fs::create_dir_all(path)?;
    }
    for path in file_paths {
        File::create(path)?;
    }
    Ok(())
}

fn make_path(ext: &str) -> Result<String, Box<dyn error::Error>> {
    match home_dir() {
        Some(home) => Ok(format!("{}/.config/ipss/{}", &home.to_str().unwrap(), ext)),
        None => Err(CustomError::new("Home folder could not be found.")),
    }
}
