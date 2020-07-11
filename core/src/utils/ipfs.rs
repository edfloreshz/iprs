pub mod installer {
  use std::path::Path;
  use std::{io, fs};
  use crate::InstallStatus;
  use std::process::Command;

  pub fn install() -> InstallStatus {
    if !is_installed() {
      if want_to_install() {
        match install_ipfs() {
          Ok(_) => {
            println!("\nIPFS is now installed.");
            InstallStatus::Installed
          },
          Err(e) => InstallStatus::Error(format!("An error occurred during the IPFS \
        installation: {}", e))
        }
      } else { InstallStatus::Error("Perhaps later...".to_string()) }
    } else {
      println!("IPFS is already installed.");
      InstallStatus::Installed
    }
  }

  fn want_to_install() -> bool {
    println!("IPSS needs IPFS to work, would you like to install it? (yes or no) ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    match input.as_str().trim() {
      "yes" | "Yes" | "YES" => {
        true
      },
      _ => {
        false
      }
    }
  }

  #[cfg(target_os = "macos")]
  pub fn is_installed() -> bool {
    Path::new("/usr/local/bin/ipfs").exists()
  }

  #[cfg(target_os = "macos")]
  fn install_ipfs() -> Result<(), String> {
    if let Some(dir) = dirs::home_dir() {
      let ipfs_download_path = dir.as_path().display().to_string() + "/.ipss/ipfs/downloads";
      match fs::create_dir_all(&ipfs_download_path) {
        Ok(_) => {
          println!("Directory created: {}", ipfs_download_path.as_str());
          let download = Command::new("curl")
              .arg("https://github.com/ipfs/go-ipfs/releases/download/v0.6.0/go-ipfs_v0.6.0_darwin-amd64.tar.gz")
              .arg(">")
              .arg("go-ipfs_v0.6.0_darwin-amd64.tar.gz")
              .current_dir(ipfs_download_path.as_str())
              .status();
          match download {
            Ok(_) => {
              let unzip = Command::new("tar")
                  .arg("-xvzf")
                  .arg("go-ipfs_v0.6.0_darwin-amd64.tar.gz")
                  .current_dir(ipfs_download_path.as_str())
                  .status();
              match unzip {
                Ok(_) =>{
                  let install = Command::new("sudo").arg("-S")
                      .arg("bash")
                      .arg("install.sh")
                      .current_dir(format!("{}/go-ipfs", ipfs_download_path.as_str()))
                      .status();
                  match install {
                    Ok(_) => {
                      // let mut bar = progress::Bar::new();
                      // bar.set_job_title(format!("Installing IPFS for {}", os::print_os_info()).as_str());
                      // for i in 0..11 {
                      //   thread::sleep(Duration::from_millis(100));
                      //   bar.reach_percent(i * 10);
                      // }
                      Ok(())
                    },
                    Err(e) => Err(e.to_string())
                  }

                },
                Err(e) => Err(e.to_string())
              }

            },
            Err(e) => Err(e.to_string()),
          }
        },
        Err(e) => Err(e.to_string())
      }
    } else {
      Err("Couldn't find your home directory".to_string())
    }
  }

  #[cfg(target_os = "windows")]
  pub fn is_installed() -> bool {
    match dirs::home_dir() {
      Some(home) => {
        let home_dir = home.as_path().display().to_string();
        Path::new(format!("{}\\Apps\\go-ipfs_v0.6.0\\go-ipfs", home_dir).as_str())
          .exists()
      },
      None => panic!("We couldn't find your home directory")
    }
  }

  #[cfg(target_os = "windows")]
  fn install_ipfs() -> Result<(), String> {
    if let Some(dir) = dirs::home_dir() {
      let ipfs_download_path = dir.as_path().display().to_string() + "/.ipss/ipfs/downloads";
      match fs::create_dir_all(&ipfs_download_path) {
        Ok(_) => {
          println!("Directory created: {}", ipfs_download_path.as_str());
          let download = Command::new("cmd")
            .args(&["/C", "powershell", "wget"])
            .arg("https://github.com/ipfs/go-ipfs/releases/download/v0.6.0/go-ipfs_v0.6.0_windows-386.zip")
            .arg("-Outfile")
            .arg("go-ipfs-v0.6.0.zip")
            .status();
          if download.unwrap().success() {
            println!("Installer downloaded.");
            let expand = Command::new("powershell")
              .arg("Expand-Archive")
              .arg("-Path")
              .arg("go-ipfs-v0.6.0.zip ")
              .arg("-DestinationPath")
              .arg("~\\Apps\\go-ipfs_v0.6.0")
              .status();
            if expand.unwrap().success() {
              if let Some(dir) = dirs::home_dir() {
                let powershell_profile = dir.as_path().display().to_string() +
                  "\\Documents\\WindowsPowerShell\\profile.ps1";
                let install_directory = dir.as_path().display().to_string() +
                  "\\Apps\\go-ipfs_v0.6.0\\go-ipfs";
                let install = Command::new("powershell")
                  .arg("Add-Content")
                  .arg(&powershell_profile)
                  .arg(format!("[System.Environment]::SetEnvironmentVariable('PATH',\
                    `$Env:PATH+';;{})", &install_directory))
                  .status();
                if install.unwrap().success() {
                  Ok(())
                } else {
                  Err("An error occurred during installation.".to_string())
                }
              } else {
                Err("We couldn't find your home directory".to_string())
              }
            } else {
              Err("We couldn't expand the IPFS installer".to_string())
            }
          } else {
            Err("We couldn't download the IPFS installer".to_string())
          }
        },
        Err(e) => Err(e.to_string())
      }
    } else {
      Err("Couldn't find your home directory".to_string())
    }
  }

  #[cfg(target_os = "linux")]
  pub fn is_installed() -> bool {
    Path::new("/usr/local/bin/ipfs").exists()
  }

  #[cfg(target_os = "linux")]
  fn install_ipfs() -> Result<(), String> {
    if let Some(dir) = dirs::home_dir() {
      let ipfs_download_path = dir.as_path().display().to_string() + "/.ipss/ipfs/downloads";
      match fs::create_dir_all(&ipfs_download_path) {
        Ok(_) => {
          println!("Directory created: {}", ipfs_download_path.as_str());
          let download = Command::new("wget").arg("https://github\
          .com/ipfs/go-ipfs/releases/download/v0.6.0/go-ipfs_v0.6\
          .0_linux-amd64.tar.gz").current_dir(ipfs_download_path.as_str()).status();
          match download {
            Ok(_) => {
              let unzip = Command::new("tar").arg("-xvzf").arg("go-ipfs_v0.6.0_linux-amd64.tar\
              .gz").current_dir(ipfs_download_path.as_str()).status();
              match unzip {
                Ok(_) =>{
                  let install = Command::new("sudo").arg("-S")
                    .arg("bash")
                    .arg("install.sh")
                    .current_dir(format!("{}/go-ipfs", ipfs_download_path.as_str()))
                    .status();
                  match install {
                    Ok(_) => {
                      // let mut bar = progress::Bar::new();
                      // bar.set_job_title(format!("Installing IPFS for {}", os::print_os_info()).as_str());
                      // for i in 0..11 {
                      //   thread::sleep(Duration::from_millis(100));
                      //   bar.reach_percent(i * 10);
                      // }
                      Ok(())
                    },
                    Err(e) => Err(e.to_string())
                  }

                },
                Err(e) => Err(e.to_string())
              }

            },
            Err(e) => Err(e.to_string()),
          }
        },
        Err(e) => Err(e.to_string())
      }
    } else {
      Err("Couldn't find your home directory".to_string())
    }
  }
}
