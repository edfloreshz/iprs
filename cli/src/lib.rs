use clap::{App, Arg, ArgMatches, SubCommand};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

pub struct ParsedArgs {
    matches: ArgMatches<'static>,
}

impl ParsedArgs {
    pub fn new() -> ParsedArgs {
        ParsedArgs {
            matches: parse_args(),
        }
    }
    pub fn call(&self) {
        if self.matches.is_present("init") {
            println!("Configuration will be initialized.");
        }
        if self.matches.is_present("daemon") {
            println!("Daemon will start.");
        }
        if self.matches.subcommand_name().is_some() {
            let subcommand_name = self.matches.subcommand_name().unwrap();
            let matches = self.matches.subcommand_matches(subcommand_name).unwrap();
            let file = matches.value_of("file").unwrap_or("No value");
            println!("{} {}", subcommand_name, file);
        }
    }
}

fn parse_args() -> ArgMatches<'static> {
    App::new("IPRS")
        .about("Global p2p file replication system.")
        .subcommand(SubCommand::with_name("init").about("Creates the required configuration."))
        .subcommand(
            SubCommand::with_name("daemon")
                .about("Scans for changes in the filesystem and updates pins in the network."),
        )
        .subcommand(
            SubCommand::with_name("add")
                .arg(
                    Arg::with_name("file")
                        .help("File to add.")
                        .required(true)
                        .index(1),
                )
                .about("Add a file to the IPFS network."),
        )
        .subcommand(
            SubCommand::with_name("get")
                .arg(
                    Arg::with_name("file")
                        .help("File to fetch.")
                        .required(true)
                        .index(1),
                )
                .about("Gets a file from the network."),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .arg(
                    Arg::with_name("file")
                        .help("File to remove.")
                      .required(true)
                      .index(1),
                )
                .about("Removes a file from other nodes."),
        )
        .get_matches()
}
