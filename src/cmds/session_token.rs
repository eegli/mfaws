use async_trait::async_trait;

use crate::{
    cmds::Command,
    config::Config,
    creds::CredentialsHandler,
    sts::{get_st_profile, session_token::SessionToken},
};

#[async_trait]
impl Command for SessionToken {
    async fn exec(mut self, config: &Config) -> anyhow::Result<()> {
        self.config.init()?;
        let creds_handler = CredentialsHandler::try_from(config)?;
        get_st_profile(self, creds_handler).await
    }
}
