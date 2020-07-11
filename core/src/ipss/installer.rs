use os_info;
use std::path::Path;
use std::process::Command;
use crate::InstallStatus;

#[cfg(target_os = "macos")]
fn run()  -> InstallStatus  {
  if ipss_installed() {
    InstallStatus::Installed
  } else {
    let install = Command::new("sudo")
        .arg("-S")
        .arg("mv")
        .arg("ipss")
        .arg("/usr/local/bin/")
        .current_dir("/Users/eduardo/Documents/GitHub/ipss/target/debug")
        .status();
    match install {
      Ok(exit) => {
        if exit.success() {
          println!("\nIPSS is now installed");
          InstallStatus::Installed
        } else {
          InstallStatus::Error(format!("We weren't able to install IPSS."))
        }
      },
      Err(e) => InstallStatus::Error(format!("An error occurred during IPSS installation: {}", e))
    }
  }
}

#[cfg(target_os = "windows")]
fn run()  -> Result<(), String> {
  println!("Installing IPSS for {}", os_info::get());
  let mut bar = progress::Bar::new();
  bar.set_job_title(format!("Installing IPSS for {}", os::print_os_info()).as_str());
  for i in 0..11 {
    thread::sleep(Duration::from_millis(100));
    bar.reach_percent(i * 10);
  }
  println!("\nIPSS is now installed.")
}

#[cfg(target_os = "linux")]
fn run() -> InstallStatus {
  if ipss_installed() {
    InstallStatus::Installed
  } else {
    let install = Command::new("sudo")
      .arg("-S")
      .arg("mv")
      .arg("ipss")
      .arg("/usr/local/bin/")
      .current_dir("/home/eduardo/Documents/GitHub/projects/ipss/target/debug")
      .status();
    match install {
      Ok(exit) => {
        if exit.success() {
          // let mut bar = progress::Bar::new();
          // bar.set_job_title(format!("Installing IPSS for {}", os::print_os_info()).as_str());
          // for i in 0..11 {
          //   thread::sleep(Duration::from_millis(100));
          //   bar.reach_percent(i * 10);
          // }
          println!("\nIPSS is now installed");
          InstallStatus::Installed
        } else {
          InstallStatus::Error(format!("We weren't able to install IPSS."))
        }
      },
      Err(e) => InstallStatus::Error(format!("An error occurred during installation: {}", e))
    }
  }

}

pub fn install() -> InstallStatus {
  match os_info::get().os_type() {
    os_info::Type::Windows => run(),
    os_info::Type::Macos => run(),
    _ => run()
  }
}

pub fn ipss_installed() -> bool {
  Path::new("/usr/local/bin/ipss").exists()
}
