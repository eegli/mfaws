use crate::cmds;
use crate::config::Config;
use clap::Parser;

#[derive(clap::Parser, Debug)]
pub enum SubCommand {
    #[clap(
        name = "assume-role",
        about = "Temporary credentials for an assumed AWS IAM Role"
    )]
    AssumeRole(cmds::AssumeRole),
    #[clap(
        name = "session-token",
        about = "Temporary credentials for an AWS IAM user"
    )]
    GetSessionToken(cmds::SessionToken),
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
    #[command(flatten)]
    pub config: Config,
}

pub fn parse() -> Cli {
    Cli::parse()
}
