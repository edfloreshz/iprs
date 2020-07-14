use std::path::{Path};
use std::process::Command;
use crate::InstallStatus;

pub fn install() -> InstallStatus {
  if ipss_installed() {
    InstallStatus::Installed
  } else {
    let install = Command::new("cargo")
      .args(&["install", "--path", "."])
      .status();
    if install.unwrap().success() {
      println!("\nIPSS is now installed");
      InstallStatus::Installed
    } else {
      InstallStatus::Error(format!("We weren't able to install IPSS."))
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
