use anyhow::Result;
mod cli;
mod cmds;
mod config;
mod creds;
mod logger;
mod profile;
mod sts;
mod utils;

use crate::creds::CredentialsHandler;
use crate::sts::StsAction;
use crate::{cli::SubCommand, profile::ProfileName};

#[macro_use]
extern crate log;

async fn run() -> Result<()> {
    let (command, config) = cli::parse()?;

    let mut creds_handler = CredentialsHandler::from_file(config.credentials.as_path())?;

    let lt_profile = creds_handler.get_long_term_profile(&config)?;

    info!("Using long-term profile \"{}\"", lt_profile.name);

    let st_profile_name = match command {
        SubCommand::AssumeRole(ref op) => op.short_profile_name(&config),
        SubCommand::GetSessionToken(ref op) => op.short_profile_name(&config),
    };

    if let Some(remaining_time) = creds_handler.is_profile_still_valid(&st_profile_name) {
        info!(
            "Found existing short-term profile \"{}\" that is valid for the next {}",
            st_profile_name, remaining_time
        );
        return Ok(());
    };

    let st_profile = match command {
        SubCommand::AssumeRole(ref op) => {
            info!("Assuming role \"{}\" for \"{}\"", op.role_arn, op.role_name);
            op.execute(&config, &lt_profile).await?
        }
        SubCommand::GetSessionToken(ref op) => {
            info!("Getting session token");
            op.execute(&config, &lt_profile).await?
        }
    };

    creds_handler.set_short_term_profile(&st_profile, &st_profile_name);
    creds_handler
        .0
        .write_to_file(config.credentials.as_path())?;

    info!(
        "Successfully added short-term credentials \"{}\"",
        st_profile_name
    );

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
