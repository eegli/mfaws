pub mod clear;
pub mod sts;

use async_trait::async_trait;

use crate::config::Config;

#[async_trait]
pub trait Command {
    async fn exec(&self, config: &Config) -> anyhow::Result<()>;
}

// All subcommands
#[derive(clap::Parser, Debug)]
pub enum SubCommand {
    #[clap(flatten)]
    StsCommand(StsCommand),
    #[clap(
        name = "clean",
        about = "Remove all temporary profiles from your credentials"
    )]
    Clean(clear::Clean),
}

// STS-specific commands
#[derive(clap::Parser, Debug)]
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

#[async_trait]
impl Command for StsCommand {
    async fn exec(&self, config: &Config) -> anyhow::Result<()> {
        self.get_credentials(config).await
    }
}
