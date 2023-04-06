use anyhow::Result;
use cmds::{Command, SubCommand};
mod cli;
mod cmds;
mod config;
mod creds;
mod logger;
mod profile;
mod sts;
mod utils;

#[macro_use]
extern crate log;

async fn run() -> Result<()> {
    let (command, config) = cli::parse()?;

    match command {
        // Sts commands are further delegated
        SubCommand::StsCommand(cmd) => cmd.exec(&config).await?,
        SubCommand::Clean(cmd) => cmd.exec(&config).await?,
    };

    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    logger::init();
    if let Err(err) = run().await {
        error!("{}", err);
    }
    Ok(())
}
