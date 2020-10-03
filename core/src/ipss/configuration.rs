use std::fs;
use dirs::home_dir;

pub fn initialize() -> std::io::Result<()> {
    let home = home_dir().unwrap();
    let path = format!("{}/.config/ipss", &home.to_str().unwrap());
    fs::create_dir_all(path)?;
    Ok(())
}