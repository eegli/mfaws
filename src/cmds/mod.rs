use crate::config::Config;
use crate::sts::{assume_role::AssumeRole, session_token::SessionToken};
use async_trait::async_trait;

pub mod clean;
pub mod sts;

#[async_trait]
pub trait Command {
    async fn exec(self, config: &Config) -> anyhow::Result<()>;
}

// All subcommands
#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    #[clap(
        name = "assume-role",
        about = "Temporary credentials for an assumed AWS IAM Role"
    )]
    AssumeRole(AssumeRole),
    #[clap(
        name = "session-token",
        about = "Temporary credentials for an AWS IAM user"
    )]
    GetSessionToken(SessionToken),
    #[clap(
        hide = true,
        about = "Remove all temporary profiles from your credentials"
    )]
    Clean(clean::Clean),
}
