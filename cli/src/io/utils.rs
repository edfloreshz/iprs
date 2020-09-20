use std::error;
use core::ipss;
use core::ipss::daemon;
use core::InstallStatus;
use std::process;
use core::replication::engine;
use std::path::Path;
use core::errors::custom::CustomError;

pub struct Config {
  pub action: ActionType,
  pub argument: String
}

enum Action {
  Single(ActionType),
  Multiple(ActionType),
  Error(ActionType)
}

pub enum ActionType {
  Init,
  Help,
  Add,
  Cat,
  Get,
  Remove,
  Daemon,
  Unknown(String)
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();
    let mut action = Action::Single(ActionType::Unknown("".to_string()));
    if let Some(arg) = args.next() {
      action = match arg.as_str() {
        "init" | "-i" => Action::Single(ActionType::Init),
        "help" | "-h" | "--help" => Action::Single(ActionType::Help),
        "add" => Action::Multiple(ActionType::Add),
        "cat" => Action::Multiple(ActionType::Cat),
        "get" => Action::Multiple(ActionType::Get),
        "remove" => Action::Multiple(ActionType::Remove),
        "daemon" => Action::Single(ActionType::Daemon),
        _ => Action::Error(ActionType::Unknown(arg.to_string()))
      }
    } else {
      help()
    }
    match action {
      Action::Single(action_type) => Ok(Config {
        action: action_type,
        argument: String::new()
      }),
      Action::Multiple(action_type) => {
        if let Some(argument) = args.next() {
          Ok(Config { action: action_type, argument })
        } else {
          help();
          process::exit(0)
        }
      },
      Action::Error(ActionType::Unknown(arg)) => {
        Ok(Config {
          action: ActionType::Unknown(arg.to_string()),
          argument: arg
        })
      },
      _ => panic!("Unhandled errors.")
    }
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
  match config.action {
    ActionType::Init => init(),
    ActionType::Help => Ok(help()),
    ActionType::Add => add(config.argument),
    ActionType::Cat => cat(config.argument),
    ActionType::Get => get(config.argument),
    ActionType::Remove => remove(),
    ActionType::Daemon => daemon(),
    ActionType::Unknown(arg) => unknown(arg),
  }
}

pub fn init() -> Result<(), Box<dyn error::Error>> {
  match ipss::installer::install() {
    InstallStatus::Installed(msg) => Ok(println!("{}", msg)),
    InstallStatus::Error(e) => Err(e)
  }
}

pub fn add(filename: String) -> Result<(), Box<dyn error::Error>> {
  engine::add(&Path::new("./").join(filename).as_path())
}

pub fn cat(filename: String) -> Result<(), Box<dyn error::Error>> {
  engine::cat(&Path::new("./").join(filename).as_path())
}

pub fn get(_id: String) -> Result<(), Box<dyn error::Error>> { Ok(()) }

pub fn remove() -> Result<(), Box<dyn error::Error>> { Ok(()) }

pub fn daemon() -> Result<(), Box<dyn error::Error>> { daemon::init() }

pub fn unknown(arg: String) -> Result<(), Box<dyn error::Error>> {
  Err(CustomError::new(format!("no such subcommand: {}", arg)))
}

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
    init [-i]       Initialize ipss local configuration.
    add <path>      Add a file to IPFS and sync it with IPSS. [Partially Implemented]
    cat <ref>       Show IPFS object details. [Not Implemented]
    get <ref>       Download IPFS objects stores in IPSS. [Not Implemented]
    remove <ref>    Remove IPFS objects from IPSS. [Not Implemented]
ADVANCED COMMANDS
  daemon            Start a long-running daemon process. [Partially Implemented]");
  process::exit(0)
}