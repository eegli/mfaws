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

#[cfg(test)]
mod test {
    use super::*;
    use crate::sts::{config::CommonStsConfig, ShortTermCredentials};

    #[test]
    fn session_token_st_profile_name() {
        let cmd = SessionToken {
            config: CommonStsConfig {
                profile_name: "test".to_string(),
                short_term_suffix: "short-term".to_string(),
                ..Default::default()
            },
        };

        assert_eq!(cmd.short_profile_name(), "test-short-term");
    }
}
