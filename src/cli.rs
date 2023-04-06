use crate::cmds::SubCommand;
use crate::config::Config;
use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
    #[command(flatten)]
    pub config: Config,
}

pub fn parse() -> anyhow::Result<(SubCommand, Config)> {
    let cli = Cli::parse();
    let (command, mut config) = (cli.command, cli.config);
    config.init()?;
    Ok((command, config))
}
