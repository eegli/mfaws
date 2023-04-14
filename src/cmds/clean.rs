use async_trait::async_trait;

use crate::{cmds::Command, config::Config, creds::CredentialsHandler};

#[derive(clap::Args, Debug, Default)]
pub struct Clean {
    #[clap(long, default_value = "false")]
    pub all: Option<bool>,
}

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
