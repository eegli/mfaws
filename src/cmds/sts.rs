use crate::cmds::Command;
use crate::sts::session_token::SessionToken;
use crate::{
    config::Config,
    creds::CredentialsHandler,
    sts::{assume_role::AssumeRole, get_st_profile},
};
use async_trait::async_trait;

#[async_trait]
impl Command for AssumeRole {
    async fn exec(mut self, config: &Config) -> anyhow::Result<()> {
        self.config.init()?;
        let creds_handler = CredentialsHandler::try_from(config)?;
        get_st_profile(self, creds_handler).await
    }
}

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
    use crate::sts::ShortTermCredentials;
    use crate::{cmds::SubCommand, sts::config::CommonStsConfig};

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
    #[test]
    fn assume_role_st_profile_name() {
        let cmd = AssumeRole {
            role_arn: "arn:aws:sts::462440:assumed-role/test-role".to_string(),
            role_name: "mfa-user".to_string(),
            config: CommonStsConfig {
                profile_name: "test".to_string(),
                short_term_suffix: "short-term".to_string(),
                ..Default::default()
            },
        };
        assert_eq!(
            cmd.short_profile_name(),
            "test_462440-assumed-role-test-role-mfa-user_short-term"
        );
    }
}
