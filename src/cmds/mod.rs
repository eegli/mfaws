use async_trait::async_trait;

use crate::{
    cmds::{clean::Clean, list::List},
    config::Config,
    sts::{assume_role::AssumeRole, session_token::SessionToken},
};

pub mod assume_role;
pub mod clean;
pub mod list;
pub mod session_token;

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
        about = "Remove short-time profiles from your credentials file. You'll be prompted to confirm the deletion"
    )]
    Clean(Clean),
    #[clap(about = "List profiles in your credentials file")]
    List(List),
}
