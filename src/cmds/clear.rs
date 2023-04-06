use async_trait::async_trait;

use crate::{cmds::Command, config::Config};

#[derive(clap::Parser, Debug, Default)]
pub struct Clear;

#[async_trait]
impl Command for Clear {
    async fn exec(&self, config: &Config) -> anyhow::Result<()> {
        Ok(())
    }
}
