use crate::{errors::custom::CustomError, Result};
use dirs::home_dir;
use std::fs;
use std::path::Path;

pub fn initialize(force: bool) -> Result<()> {
    if !force {
        if Path::new(&make_path("")?).exists() {
            return Err(CustomError::new(
                "Configuration already exists, try using -f to \
            reinitialize, this will delete any previous configuration and files in your account.",
            ));
        }
    }
    make_config()
}

fn make_config() -> Result<()> {
    let config_paths = vec![make_path("config")?, make_path("database")?];
    let file_paths = vec![make_path("database/files.db")?];
    for path in config_paths {
        fs::create_dir_all(path)?;
    }
    for path in file_paths {
        fs::File::create(path)?;
    }
    Ok(())
}

fn make_path(ext: &str) -> Result<String> {
    match home_dir() {
        Some(home) => Ok(format!("{}/.config/iprs/{}", &home.to_str().unwrap(), ext)),
        None => Err(CustomError::new("Home folder could not be found.")),
    }
}
