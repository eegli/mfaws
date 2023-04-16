use clap::Parser;

use crate::{cmds::SubCommand, config::Config};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
    #[command(flatten)]
    pub config: Config,
}

pub fn parse() -> (SubCommand, Config) {
    let cli = Cli::parse();
    (cli.command, cli.config)
}
