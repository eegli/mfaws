use async_trait::async_trait;

use crate::{
    config::Config,
    profile::{LongTermProfile, ProfileName, ShortTermProfile},
    sts::{extract_sts_err, StsAction},
};

#[derive(clap::Parser, Debug, Default)]
pub struct SessionToken;

impl ProfileName for SessionToken {
    fn short_profile_name(&self, config: &Config) -> String {
        format!("{}-{}", config.profile_name, config.short_term_suffix)
    }
}

#[async_trait]
impl<'a> StsAction for &'a SessionToken {
    type Output = ShortTermProfile<'a>;

    const DEFAULT_DURATION: i32 = 43200;

    async fn execute(
        &self,
        config: &Config,
        lt_profile: &LongTermProfile,
    ) -> Result<Self::Output, anyhow::Error> {
        let mfa_token = self.get_mfa_token()?;
        let output = lt_profile
            .create_client()
            .await
            .get_session_token()
            .serial_number(lt_profile.mfa_device.to_string())
            .duration_seconds(config.duration.unwrap_or(Self::DEFAULT_DURATION))
            .token_code(mfa_token.to_string())
            .send()
            .await
            .map_err(extract_sts_err)?;
        let short_term_profile = ShortTermProfile::try_from(output.credentials)?;
        Ok(short_term_profile)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn session_token_st_profile_name() {
        let role = SessionToken::default();
        let conf = Config {
            profile_name: "test".to_string(),
            short_term_suffix: "short-term".to_string(),
            ..Default::default()
        };
        assert_eq!(role.short_profile_name(&conf), "test-short-term");
    }
}
