use async_trait::async_trait;

use crate::{cmds::Command, config::Config};

#[derive(clap::Parser, Debug, Default)]
pub struct Clean;

#[async_trait]
impl Command for Clean {
    async fn exec(&self, config: &Config) -> anyhow::Result<()> {
        let creds_handler = config.credentials_handler()?;
        let sections = creds_handler.0.sections().flatten().collect::<Vec<_>>();
        info!("Sections: {:#?}", sections);
        // TODO
        Ok(())
    }
}
