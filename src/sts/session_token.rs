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

    async fn get_credentials(
        &self,
        config: &Config,
        lt_profile: &LongTermProfile,
    ) -> anyhow::Result<ShortTermProfile> {
        let mfa_token = self.get_mfa_token()?;
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
}
