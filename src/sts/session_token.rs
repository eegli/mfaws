use crate::{
    profile::{LongTermProfile, ShortTermProfile},
    sts::{config::CommonStsConfig, extract_sts_err, ShortTermCredentials},
};

#[derive(clap::Args, Debug, Default)]
pub struct SessionToken {
    #[clap(flatten)]
    pub config: CommonStsConfig,
}

impl ShortTermCredentials for SessionToken {
    const DEFAULT_DURATION: i32 = 43200;

    fn short_profile_name(&self) -> String {
        format!(
            "{}-{}",
            self.config.profile_name, self.config.short_term_suffix
        )
    }

    fn config<'c>(&'c self) -> &'c CommonStsConfig {
        &self.config
    }

    fn log_action(&self) {
        info!("Getting session token");
    }

    #[cfg(not(feature = "e2e_test"))]
    async fn get_credentials(
        &self,
        config: &CommonStsConfig,
        mfa_token: String,
        lt_profile: &LongTermProfile<'_>,
    ) -> anyhow::Result<ShortTermProfile> {
        let output = lt_profile
            .create_client()
            .await
            .get_session_token()
            .serial_number(lt_profile.mfa_device.to_string())
            .duration_seconds(config.duration.unwrap_or(Self::DEFAULT_DURATION))
            .token_code(mfa_token)
            .send()
            .await
            .map_err(extract_sts_err)?;
        let short_term_profile = ShortTermProfile::try_from(output.credentials)?;
        Ok(short_term_profile)
    }

    #[cfg(feature = "e2e_test")]
    async fn get_credentials(
        &self,
        config: &CommonStsConfig,
        mfa_token: String,
        lt_profile: &LongTermProfile,
    ) -> anyhow::Result<ShortTermProfile> {
        Ok(ShortTermProfile {
            access_key: "sts-access-key".to_owned(),
            secret_key: "sts-secret-key".to_owned(),
            session_token: "sts-session-token".to_owned(),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn short_profile_name() {
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
