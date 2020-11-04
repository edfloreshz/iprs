use crate::Result;
use core::errors::custom::CustomError;
use core::iprs;
use core::iprs::daemon;
use core::replication::engine;
use std::env::Args;
use std::path::Path;
use std::{process, env};

pub struct Config {
    pub config: Command,
}

pub enum Command {
    Help,
    Version,
    Init(Options),
    Add(Options),
    Cat(Options),
    Get(Options),
    Remove(Options),
    Daemon(Options),
    Unknown(String),
}

#[derive(Clone)]
pub struct Options {
    force: bool,
    input: Option<Vec<String>>,
}

impl Options {
    fn new(force: bool, input: Option<Vec<String>>) -> Options {
        Options { force, input }
    }
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config> {
        args.next();
        let mut subcommand = String::new();
        match args.next() {
            Some(arg) => subcommand = arg,
            None => help(),
        }
        match subcommand.as_str() {
            "help" => Ok(Config {
                config: Command::Help,
            }),
            "version" => Ok(Config {
                config: Command::Version,
            }),
            _ => {
                let options = get_options(&mut args);
                match subcommand.as_str() {
                    "init" => Ok(Config {
                        config: Command::Init(options.clone()),
                    }),
                    "add" => Ok(Config {
                        config: Command::Add(options.clone()),
                    }),
                    "cat" => Ok(Config {
                        config: Command::Cat(options.clone()),
                    }),
                    "get" => Ok(Config {
                        config: Command::Get(options.clone()),
                    }),
                    "remove" => Ok(Config {
                        config: Command::Remove(options.clone()),
                    }),
                    "daemon" => Ok(Config {
                        config: Command::Daemon(options.clone()),
                    }),
                    _ => Ok(Config {
                        config: Command::Unknown(subcommand.to_string()),
                    }),
                }
            }
        }
    }
}

fn get_options(args: &mut Args) -> Options {
    let (mut flags, mut input) = (vec![], vec![]);
    while let Some(arg) = args.next() {
        if arg.starts_with("-") {
            flags.push(arg)
        } else {
            input.push(arg)
        }
    }
    let mut options = Options::new(false, Some(input));
    if flags.contains(&"-f".to_string()) {
        options.force = true
    }
    options
}

pub fn run(config: Config) -> Result<()> {
    match config.config {
        Command::Init(options) => init(options),
        Command::Help => Ok(help()),
        Command::Version => Ok(version()),
        Command::Add(options) => add(options),
        Command::Cat(options) => cat(options),
        Command::Get(options) => get(options),
        Command::Remove(options) => remove(options),
        Command::Daemon(_) => daemon(),
        Command::Unknown(arg) => unknown(arg),
    }
}

fn init(options: Options) -> Result<()> {
    match iprs::configuration::initialize(options.force) {
        Ok(()) => Ok(println!("Configuration initialized correctly.")),
        Err(e) => Err(e),
    }
}

fn add(options: Options) -> Result<()> {
    match options.input {
        Some(input) => {
            let current_dir = &env::current_dir()?;
            let path = input
                .iter()
                .map(|file_name| Path::new(current_dir).join(file_name))
                .collect();
            engine::add(path)
        },
        None => Err(CustomError::new("No input was provided.")),
    }
}

fn cat(options: Options) -> Result<()> {
    match options.input {
        Some(input) => {
            if input.len() == 1 {
                engine::cat(Path::new("./").join(input[0].clone()))
            } else {
                Err(CustomError::new("More than one file was provided."))
            }
        }
        None => Err(CustomError::new("No input was provided")),
    }
}

fn get(options: Options) -> Result<()> {
    match options.input {
        Some(input) => engine::get(input),
        None => Err(CustomError::new("No input was provided.")),
    }
}

fn remove(options: Options) -> Result<()> {
    match options.input {
        Some(input) => engine::remove(input),
        None => Err(CustomError::new("No input was provided.")),
    }
}

fn daemon() -> Result<()> {
    daemon::init()
}

fn version() {
    println!("IPRS v0.1.1")
}

fn unknown(arg: String) -> Result<()> {
    let error: &str = &format!("no such subcommand: {}", arg).to_owned()[..];
    Err(CustomError::new(error))
}

fn help() {
    println!(
        "\n
        ██╗██████╗ ██████╗ ███████╗
        ██║██╔══██╗██╔══██╗██╔════╝
        ██║██████╔╝██████╔╝███████╗
        ██║██╔═══╝ ██║  ██╗╚════██║
        ██║██║     ██║  ██║███████║
        ╚═╝╚═╝     ╚═╝  ╚═╝╚══════╝
\n
Welcome to the InterPlanetary Sync System! \n
USAGE
  iprs - Global p2p file replication system

  iprs help - Show subcommands.

SUBCOMMANDS
  BASIC COMMANDS
    version         Prints the current version.
    init [-f]       Initialize IPRS local configuration. [-f] to force reinitialization.
    add <path>      Add a file to IPFS and sync it with IPRS. [Partially Implemented]
    cat <ref>       Show IPFS object details. [Not Implemented]
    get <ref>       Download IPFS objects stores in IPRS. [Not Implemented]
    remove <ref>    Remove IPFS objects from IPRS. [Not Implemented]
ADVANCED COMMANDS
  daemon            Start a long-running daemon process. [Partially Implemented]"
    );
    process::exit(0)
}
