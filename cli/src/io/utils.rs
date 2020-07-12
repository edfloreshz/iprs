use std::error::Error;
use core::utils::{ipfs};
use core::ipss;
use core::ipss::daemon;
use core::InstallStatus;
use std::process;

pub struct Config {
    pub action: ActionType,
    pub argument: String
}

enum Action {
    Single(ActionType),
    Multiple(ActionType)
}

pub enum ActionType {
    Init,
    Help,
    Add,
    Cat,
    Get,
    Remove,
    Daemon,
    Unknown
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let mut action = Action::Single(ActionType::Unknown);
        if let Some(arg) = args.next() {
            action = match arg.as_str() {
                "init" | "-i" => Action::Single(ActionType::Init),
                "help" | "-h" | "--help" => Action::Single(ActionType::Help),
                "add" => Action::Multiple(ActionType::Add),
                "cat" => Action::Multiple(ActionType::Cat),
                "get" => Action::Multiple(ActionType::Get),
                "remove" => Action::Multiple(ActionType::Remove),
                "daemon" => Action::Single(ActionType::Daemon),
                _ => Action::Single(ActionType::Unknown)
            }
        } else {
            help()
        }
        match action {
            Action::Single(action_type) => Ok(Config { action: action_type, argument: String::new() }),
            Action::Multiple(action_type) => {
                if let Some(argument) = args.next() {
                    Ok(Config { action: action_type, argument })
                } else {
                    help();
                    process::exit(1)
                }
            }
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.action {
        ActionType::Init => init(),
        ActionType::Help => help(),
        ActionType::Add => add(),
        ActionType::Cat => cat(),
        ActionType::Get => get(),
        ActionType::Remove => remove(),
        ActionType::Daemon => daemon(),
        ActionType::Unknown => println!("Unknown parameter"),
    }
    Ok(())
}

pub fn init() {
    match ipfs::installer::install() {
        InstallStatus::Installed => {
            match ipss::installer::install() {
                InstallStatus::Installed => {
                    println!("IPSS is already installed.\n\
                    Run ipss daemon to start the daemon");

                },
                InstallStatus::Error(e) => println!("{}", e)
            }
        }
        InstallStatus::Error(e) => println!("{}", e),
    }
}

pub fn add() {}

pub fn cat() {}

pub fn get() {}

pub fn remove() {}

pub fn daemon() { daemon::init() }

pub fn unknown() {}

pub fn help() {
    println!("\n
        ██╗██████╗ ███████╗███████╗
        ██║██╔══██╗██╔════╝██╔════╝
        ██║██████╔╝███████╗███████╗
        ██║██╔═══╝ ╚════██║╚════██║
        ██║██║     ███████║███████║
        ╚═╝╚═╝     ╚══════╝╚══════╝
\n
Welcome to the InterPlanetary Sync System! \n
USAGE
  ipss - Global p2p file replication system

  ipfs [help] [--help] [-h] <command> ...

SUBCOMMANDS
  BASIC COMMANDS
    init [-i]       Initialize ipss local configuration. [Partially Implemented]
    add <path>      Add a file to IPFS and sync it with IPSS. [Not Implemented]
    cat <ref>       Show IPFS object details. [Not Implemented]
    get <ref>       Download IPFS objects stores in IPSS. [Not Implemented]
    remove <ref>    Remove IPFS objects from IPSS. [Not Implemented]
ADVANCED COMMANDS
  daemon            Start a long-running daemon process. [Partially Implemented]");
    process::exit(1)
}