#![cfg_attr(
    feature = "e2e_test",
    allow(dead_code, unused_imports, unused_variables)
)]
mod cli;
mod cmds;
mod config;
mod creds;
mod logger;
mod profile;
mod sts;
mod utils;

use cmds::{Command, SubCommand};

#[macro_use]
extern crate log;

async fn run() -> anyhow::Result<()> {
    let (command, config) = cli::parse()?;
    match command {
        SubCommand::AssumeRole(cmd) => cmd.exec(&config).await?,
        SubCommand::GetSessionToken(cmd) => cmd.exec(&config).await?,
        SubCommand::Clean(cmd) => cmd.exec(&config).await?,
    };
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    if let Err(err) = run().await {
        error!("{}", err);
    }
    Ok(())
}
