use crate::{
    cmds::sts::AssumeRole,
    config::Config,
    profile::{LongTermProfile, ShortTermProfile},
    sts::{extract_sts_err, StsCredentialsRequest},
};
use async_trait::async_trait;
use std::borrow::Cow;

#[async_trait]
impl StsCredentialsRequest for AssumeRole {
    const DEFAULT_DURATION: i32 = 3600;

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
            .assume_role()
            .set_role_arn(Some(self.role_arn.clone()))
            .set_role_session_name(Some(self.role_name.clone()))
            .set_serial_number(Some(lt_profile.mfa_device.to_string()))
            .set_token_code(Some(mfa_token))
            .set_duration_seconds(config.duration.or(Some(Self::DEFAULT_DURATION)))
            .send()
            .await
            .map_err(extract_sts_err)?;

        let mut stp = ShortTermProfile::try_from(output.credentials)?;

        // Assumed_role_arn is the user input role_arn, not the actual
        // role_arn returned by STS
        stp.assumed_role_arn = Some(Cow::Borrowed(&self.role_arn));
        // Assumed_role_id is the actual role_id returned by STS
        stp.assumed_role_id = output
            .assumed_role_user
            .map(|v| v.assumed_role_id)
            .unwrap_or_default();

        Ok(stp)
    }

    #[cfg(feature = "e2e_test")]
    async fn get_credentials(
        &self,
        _config: &Config,
        _mfa_token: String,
        _lt_profile: &LongTermProfile,
    ) -> anyhow::Result<ShortTermProfile> {
        Ok(ShortTermProfile::default())
    }
}