pub mod installer {
  use std::path::Path;
  use std::fs;
  use std::io;
  use std::process::Command;

  pub enum InstallStatus {
    Installed,
    Error(String)
  }

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

  #[cfg(target_os = "windows")]
  pub fn is_installed() -> bool {
    match dirs::home_dir() {
      Some(home) => {
        Path::new(format!("{}\\Apps\\go-ipfs_v0.6.0\\go-ipfs", home).as_str()).exists()
      },
      None => panic!("We couldn't find your home directory")
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
