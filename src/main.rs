use core::utils::{ipfs};
use core::ipss;
use core::ipss::daemon;
use core::InstallStatus;

fn main() {
    print_logo();
    match ipfs::installer::install() {
        InstallStatus::Installed => {
            match ipss::installer::install() {
                InstallStatus::Installed => {
                    println!("IPSS is already installed.");
                    daemon::init();
                },
                InstallStatus::Error(e) => println!("{}", e)
            }
        }
        InstallStatus::Error(e) => println!("{}", e),
    }
}

fn print_logo() {
    println!("\n
        ██╗██████╗ ███████╗███████╗
        ██║██╔══██╗██╔════╝██╔════╝
        ██║██████╔╝███████╗███████╗
        ██║██╔═══╝ ╚════██║╚════██║
        ██║██║     ███████║███████║
        ╚═╝╚═╝     ╚══════╝╚══════╝
    \nWelcome to the InterPlanetary Sync System!
    ")
}