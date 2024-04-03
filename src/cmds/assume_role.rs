use crate::{
    cmds::Command,
    config::Config,
    creds::CredentialsHandler,
    sts::{assume_role::AssumeRole, get_st_profile},
};

impl Command for AssumeRole {
    async fn exec(mut self, config: &Config) -> anyhow::Result<()> {
        self.config.init()?;
        let creds_handler = CredentialsHandler::try_from(config)?;
        get_st_profile(self, creds_handler).await
    }
}
