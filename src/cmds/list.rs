use async_trait::async_trait;

use crate::{cmds::Command, config::Config, creds::CredentialsHandler};

#[derive(clap::Args, Debug, Default)]
pub struct List;

#[async_trait]
impl Command for List {
    async fn exec(self, config: &Config) -> anyhow::Result<()> {
        let creds_handler = CredentialsHandler::try_from(config)?;
        let sections = creds_handler.ini.sections().flatten().collect::<Vec<_>>();
        let info = match sections.len() {
            0 => format!("No AWS credential profiles found"),
            1 => format!("Found 1 AWS credential profile:"),
            _ => format!("Found {} AWS credential profiles:", sections.len()),
        };
        info!("{info}");
        for section in sections {
            println!("{}", section);
        }

        Ok(())
    }
}
