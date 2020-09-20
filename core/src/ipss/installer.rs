use std::path::{Path};
use std::process::Command;
use crate::InstallStatus;
use crate::errors::custom::CustomError;

pub fn install() -> InstallStatus {
  if ipss_installed() {
    InstallStatus::Installed("IPSS is already initialized \nRun 'ipss daemon' to start the \
    daemon".to_string())
  } else {
    let install = Command::new("cargo")
      .args(&["install", "--path", "."])
      .status();
    if install.unwrap().success() {
      InstallStatus::Installed("\nIPSS is now installed".to_string())
    } else {
      InstallStatus::Error(CustomError::new("We weren't able to install IPSS.".to_string()))
    }
  }
}

#[cfg(not(target_os = "windows"))]
pub fn ipss_installed() -> bool {
  if let Some(dir) = dirs::home_dir() {
    let path = dir.as_path().display().to_string() + "/.cargo/bin/ipss";
    Path::new(path.as_str()).exists()
  } else {
    panic!("We weren't able to find your home directory.")
  }
}

#[cfg(target_os = "windows")]
pub fn ipss_installed() -> bool {
  if let Some(dir) = dirs::home_dir() {
    let path = dir.as_path().display().to_string() + "\\.cargo\\bin\\ipss";
    Path::new(path.as_str()).exists()
  } else {
    panic!("We weren't able to find your home directory.")
  }
}
