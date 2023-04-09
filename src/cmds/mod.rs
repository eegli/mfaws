pub mod clean;
pub mod sts;

use crate::config::Config;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    async fn exec(&self, config: &Config) -> anyhow::Result<()>;
}

// All subcommands
#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    #[clap(flatten)]
    StsCommand(StsCommand),
    #[clap(
        hide = true,
        about = "Remove all temporary profiles from your credentials"
    )]
    Clean(clean::Clean),
}

// STS-specific commands
#[derive(clap::Subcommand, Debug)]
pub enum StsCommand {
    #[clap(
        name = "assume-role",
        about = "Temporary credentials for an assumed AWS IAM Role"
    )]
    AssumeRole(sts::AssumeRole),
    #[clap(
        name = "session-token",
        about = "Temporary credentials for an AWS IAM user"
    )]
    GetSessionToken(sts::SessionToken),
}
