use core::utils::{ipfs};
use core::ipss;
use std::path::Path;
use core::ipss::daemon;
use std::result::Result::Err;
use ipfs::installer::InstallStatus;

fn main() {
    println!("\nWelcome to the InterPlanetary Sync System!");
    match ipfs::installer::install() {
        Ok(_) => {
            match ipss::installer::install() {
                Ok(_) => daemon::init(),
                Err(e) => println!("{}", e)
            }
        },
        Err(InstallStatus::Installed) => {
            if ipss_installed() {
                daemon::init();
            }
            match ipss::installer::install() {
                Ok(_) => daemon::init(),
                Err(e) => println!("{}", e)
            }
        }
        Err(InstallStatus::Error(e)) => println!("{}", e),
    }
}

fn ipss_installed() -> bool {
    Path::new("/usr/local/bin/ipss").exists()
}
