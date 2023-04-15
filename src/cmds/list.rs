use async_trait::async_trait;

use crate::{cmds::Command, config::Config, creds::CredentialsHandler};

#[derive(clap::Args, Debug, Default)]
pub struct List;

#[async_trait]
impl Command for List {
    async fn exec(self, config: &Config) -> anyhow::Result<()> {
        let creds_handler = CredentialsHandler::try_from(config)?;
        let sections = creds_handler.ini.sections().flatten().collect::<Vec<_>>();
        info!("Found {} AWS credential profiles:", sections.len());
        for section in sections {
            println!("{}", section);
        }

        Ok(())
    }
}
