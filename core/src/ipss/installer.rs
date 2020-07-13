use os_info;
use std::path::{Path};
use std::process::Command;
use crate::InstallStatus;

#[cfg(target_os = "macos")]
fn run()  -> InstallStatus  {
  if ipss_installed() {
    InstallStatus::Installed
  } else {
    let install = Command::new("sudo")
        .arg("-S")
        .arg("cp")
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
fn run() -> InstallStatus {
  if ipss_installed() {
    InstallStatus::Installed
  } else {
    let install = Command::new("cmd")
      .args(&["/C", "powershell", "cp", "ipss.exe", "~\\Apps\\ipss\\"])
      .current_dir("C:\\Users\\Eduardo Flores\\CLionProjects\\ipss\\target\\debug")
      .status();
    if install.unwrap().success() {
      if let Some(dir) = dirs::home_dir() {
        let install_dir = dir.as_path().display().to_string() + "\\Apps\\ipss";
        match fs::create_dir_all(PathBuf::from(install_dir)) {
          Ok(_) => {
            let execution_policy = Command::new("powershell")
              .args(&["Start-Process","powershell","-Verb","runAs"])
              .status();
            if execution_policy.unwrap().success() {
              let powershell_profile = dir.as_path().display().to_string() +
                "\\Documents\\WindowsPowerShell\\profile.ps1";
              let install_directory = dir.as_path().display().to_string() +
                "\\Apps\\ipss";
              let env = format!("\"[System.Environment]::SetEnvironmentVariable('PATH',\
            $Env:PATH+';;\
            {}')\"", &install_directory);
              println!("Add-Content {} {}", powershell_profile, env);
              let access = Command::new("powershell")
                .arg("(get-acl c:\\).Access").status();
              if access.unwrap().success() {
                let add_env_variable = Command::new("powershell")
                  .arg("Add-Content")
                  .arg(&powershell_profile)
                  .arg(&env)
                  .status();
                if add_env_variable.unwrap().success(){
                  InstallStatus::Installed
                } else {
                  InstallStatus::Error("We weren't able to create environment variable.".to_string())
                }
              } else {
                InstallStatus::Error("Access to directory denied.".to_string())
              }
            } else {
              InstallStatus::Error("Could not set execution policy.".to_string())
            }
          },
          Err(e) => InstallStatus::Error(e.to_string())
        }
      } else {
        InstallStatus::Error("We couldn't find your home directory".to_string())
      }
    } else {
      InstallStatus::Error("We weren't able to install IPSS".to_string())
    }
  }
}

#[cfg(target_os = "linux")]
fn run() -> InstallStatus {
  if ipss_installed() {
    InstallStatus::Installed
  } else {
    let install = Command::new("sudo")
      .arg("-S")
      .arg("cp")
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

#[cfg(not(target_os = "windows"))]
pub fn ipss_installed() -> bool {
  Path::new("/usr/local/bin/ipss").exists()
}

#[cfg(target_os = "windows")]
pub fn ipss_installed() -> bool {
  Path::new("/usr/local/bin/ipss").exists()
}
