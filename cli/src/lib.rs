use clap::{App, ArgMatches, SubCommand};

pub struct ParsedArgs {
    matches: ArgMatches<'static>,
}

impl ParsedArgs {
    pub fn new() -> ParsedArgs {
        ParsedArgs { matches: parse_args() }
    }
    pub fn call(&self) {
        if self.matches.is_present("init") {
            println!("Configuration will be initialized.");
        }
        if self.matches.is_present("daemon") {
            println!("Daemon will start.");
        }
    }
}

fn parse_args() -> ArgMatches<'static> {
    App::new("IPRS")
        .version("0.1.0")
        .author("Eduardo F. <edfloreshz@gmail.com>")
        .about("Global p2p file replication system.")
        .subcommand(SubCommand::with_name("init").about("Creates the required configuration."))
        .subcommand(
            SubCommand::with_name("daemon")
                .about("Scans for changes in the filesystem and updates pins in the network."),
        )
        .get_matches()
}
