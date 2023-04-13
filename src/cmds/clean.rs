use async_trait::async_trait;

use crate::{clean::Clean, cmds::Command, config::Config, creds::CredentialsHandler};

#[async_trait]
impl Command for Clean {
    async fn exec(self, config: &Config) -> anyhow::Result<()> {
        let creds_handler = CredentialsHandler::try_from(config)?;
        let sections = creds_handler.ini.sections().flatten().collect::<Vec<_>>();
        info!("Sections: {:#?}", sections);
        // TODO
        Ok(())
    }
}
