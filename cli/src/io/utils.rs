use std::error;
use core::ipss;
use core::ipss::daemon;
use std::process;
use core::replication::engine;
use std::path::{Path};
use core::errors::custom::CustomError;

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
  Unknown(String)
}

#[derive(Clone)]
pub struct Options {
  force: bool,
  input: Option<Vec<String>>
}

impl Options {
  fn new(force: bool, input: Option<Vec<String>>) -> Options {
    Options { force, input }
  }
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, Box<dyn error::Error>> {
    args.next();
    let mut subcommand = String::new();
    let input = Some(vec![]);
    let mut options= Options::new(false, input.clone());
    let mut arguments= vec![];

    match args.next() {
      Some(arg) => subcommand = arg,
      None => help()
    }
    match subcommand.as_str() {
      "help" => Ok(Config { config: Command::Help}),
      "version" => Ok(Config {config: Command::Version}),
      _ => {
        while let Some(arg) = args.next() {
          arguments.push(arg)
        }
        let mut options_args = arguments.clone();
        options_args.retain(|arg| arg.starts_with("-"));
        let mut input_arguments = arguments.clone();
        input_arguments.retain(|arg| !arg.starts_with("-"));
        // if options_args.is_empty() && input_arguments.is_empty() {
        //   return Err(CustomError::new("None of the options provided are valid.".to_string()))
        // }
        if options_args.contains(&"-f".to_string()) { options.force = true }
        options.input = Some(input_arguments);
        match subcommand.as_str() {
          "init" => Ok(Config { config: Command::Init(options.clone())}),
          "add" => Ok(Config { config: Command::Add(options.clone())}),
          "cat" => Ok(Config { config: Command::Cat(options.clone())}),
          "get" => Ok(Config { config: Command::Get(options.clone())}),
          "remove" => Ok(Config { config: Command::Remove(options.clone())}),
          "daemon" => Ok(Config { config: Command::Daemon(options.clone())}),
          _ => Ok(Config { config: Command::Unknown(subcommand.to_string())})
        }
      }
    }
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
  match config.config {
    Command::Init(options)    => init(options),
    Command::Help => Ok(help()),
    Command::Version => Ok(version()),
    Command::Add(options)     => add(options),
    Command::Cat(options)     => cat(options),
    Command::Get(options)     => get(options),
    Command::Remove(options)  => remove(options),
    Command::Daemon(_)                => daemon(),
    Command::Unknown(arg)       => unknown(arg),
  }
}

pub fn init(options: Options) -> Result<(), Box<dyn error::Error>> {
  match ipss::configuration::initialize(options.force) {
    Ok(()) => Ok(println!("Configuration initialized correctly.")),
    Err(e) => Err(e)
  }
}

pub fn add(options: Options) -> Result<(), Box<dyn error::Error>> {
  match options.input {
    Some(input) => engine::add(input),
    None => Err(CustomError::new("No input was provided.".to_string()))
  }
}

pub fn cat(options: Options) -> Result<(), Box<dyn error::Error>> {
  match options.input {
    Some(input) => if input.len() == 1 {
      engine::cat(Path::new("./").join(input[0].clone()))
    } else {
      Err(CustomError::new("More than one file was provided.".to_string()))
    },
    None => Err(CustomError::new("No input was provided".to_string()))
  }
}

pub fn get(options: Options) -> Result<(), Box<dyn error::Error>> {
  match options.input {
    Some(input) => engine::get(input),
    None => Err(CustomError::new("No input was provided.".to_string()))
  }
}

pub fn remove(options: Options) -> Result<(), Box<dyn error::Error>> {
  match options.input {
    Some(input) => engine::remove(input),
    None => Err(CustomError::new("No input was provided.".to_string()))
  }
}

pub fn daemon() -> Result<(), Box<dyn error::Error>> { daemon::init() }

pub fn version() {
  println!("IPSS v0.1.1")
}

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

  ipfs help - Show subcommands.

SUBCOMMANDS
  BASIC COMMANDS
    init [-f]       Initialize ipss local configuration. [-f] to force reinitialization.
    add <path>      Add a file to IPFS and sync it with IPSS. [Partially Implemented]
    cat <ref>       Show IPFS object details. [Not Implemented]
    get <ref>       Download IPFS objects stores in IPSS. [Not Implemented]
    remove <ref>    Remove IPFS objects from IPSS. [Not Implemented]
ADVANCED COMMANDS
  daemon            Start a long-running daemon process. [Partially Implemented]");
  process::exit(0)
}