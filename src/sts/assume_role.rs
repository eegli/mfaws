use std::borrow::Cow;

use crate::{
    profile::{LongTermProfile, ShortTermProfile},
    sts::{config::CommonStsConfig, extract_sts_err, ShortTermCredentials},
};

#[derive(clap::Args, Debug, Default)]
pub struct AssumeRole {
    #[arg(
        long = "role-arn",
        env = "AWS_ROLE_ARN",
        help = "The ARN of the AWS IAM Role you want to assume"
    )]
    pub role_arn: String,
    #[arg(
        long = "role-session-name",
        default_value = "mfa-user",
        env = "AWS_ROLE_SESSION_NAME",
        help = "Custom friendly session name when assuming a role"
    )]
    pub role_name: String,
    #[clap(flatten)]
    pub config: CommonStsConfig,
}

impl ShortTermCredentials for AssumeRole {
    const DEFAULT_DURATION: i32 = 3600;

    fn short_profile_name(&self) -> String {
        let arn = self
            .role_arn
            .split([':', '/'])
            .skip(4)
            .collect::<Vec<&str>>()
            .join("-");
        self.config.profile_name.clone()
            + "_"
            + &arn
            + "-"
            + &self.role_name
            + "_"
            + &self.config.short_term_suffix
    }

    fn config<'c>(&'c self) -> &'c CommonStsConfig {
        &self.config
    }

    fn log_action(&self) {
        info!(
            "Assuming role \"{}\" for \"{}\"",
            self.role_arn, self.role_name
        );
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
        stp.assumed_role_id = output.assumed_role_user.map(|v| v.assumed_role_id);

        Ok(stp)
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
            assumed_role_id: Some(self.role_arn.to_string()),
            assumed_role_arn: Some(Cow::Owned("1111/user".to_owned())),
            ..Default::default()
        })
    }
}
