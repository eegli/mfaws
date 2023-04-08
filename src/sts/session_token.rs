use crate::{
    cmds::sts::SessionToken,
    config::Config,
    profile::{LongTermProfile, ShortTermProfile},
    sts::{extract_sts_err, StsCredentialsRequest},
};
use async_trait::async_trait;

#[async_trait]
impl StsCredentialsRequest for SessionToken {
    const DEFAULT_DURATION: i32 = 43200;

    #[cfg(not(feature = "e2e_test"))]
    async fn get_credentials(
        &self,
        config: &Config,
        mfa_token: String,
        lt_profile: &LongTermProfile,
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
        config: &Config,
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
