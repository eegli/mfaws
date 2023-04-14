use async_trait::async_trait;

use crate::utils::confirm_prompt;
use crate::{cmds::Command, config::Config, creds::CredentialsHandler};

#[derive(clap::Args, Debug, Default)]
pub struct Clean {
    #[arg(
        long = "short-term-suffix",
        default_value = "short-term",
        help = "To identify the auto-generated short-term credential profile"
    )]
    pub short_term_suffix: String,
}

#[async_trait]
impl Command for Clean {
    async fn exec(self, config: &Config) -> anyhow::Result<()> {
        let mut creds_handler = CredentialsHandler::try_from(config)?;
        let sections =
            creds_handler.get_profiles_matching(|p| p.ends_with(&self.short_term_suffix));
        if sections.is_empty() {
            info!("No short-term profiles found");
            return Ok(());
        }
        info!("Do you want to delete the following short-term profiles?");
        for section in &sections {
            println!("{}", section);
        }
        if confirm_prompt("Confirm deletion") {
            for section in sections {
                creds_handler.ini.delete(Some(&section));
            }
            info!("Successfully deleted short-term profiles");
            creds_handler.to_file()?;
        } else {
            info!("Aborted deletion");
        }
        Ok(())
    }
}
