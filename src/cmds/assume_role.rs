use crate::{
    config::Config,
    profile::{LongTermProfile, ProfileName, ShortTermProfile},
    sts::{extract_sts_err, StsAction},
};
use async_trait::async_trait;
use std::borrow::Cow;

#[derive(clap::Parser, Debug, Default)]
pub struct AssumeRole {
    #[arg(
        long = "role-arn",
        help = "The ARN of the AWS IAM Role you want to assume"
    )]
    pub role_arn: String,
    #[arg(
        long = "role-session-name",
        default_value = "mfa-user",
        help = "Custom friendly session name when assuming a role"
    )]
    pub role_name: String,
}

impl ProfileName for AssumeRole {
    fn short_profile_name(&self, config: &Config) -> String {
        let arn = self
            .role_arn
            .split([':', '/'])
            .skip(4)
            .collect::<Vec<&str>>()
            .join("-");
        config.profile_name.clone()
            + "_"
            + &arn
            + "-"
            + &self.role_name
            + "_"
            + &config.short_term_suffix
    }
}

#[async_trait]
impl<'a> StsAction for &'a AssumeRole {
    type Output = ShortTermProfile<'a>;

    const DEFAULT_DURATION: i32 = 3600;

    async fn execute(
        &self,
        config: &Config,
        lt_profile: &LongTermProfile,
    ) -> Result<Self::Output, anyhow::Error> {
        let mfa_token = self.get_mfa_token()?;
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
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn assume_role_st_profile_name() {
        let role = AssumeRole {
            role_arn: "arn:aws:sts::462440:assumed-role/test-role".to_string(),
            role_name: "mfa-user".to_string(),
        };
        let conf = Config {
            profile_name: "test".to_string(),
            short_term_suffix: "short-term".to_string(),
            ..Default::default()
        };
        assert_eq!(
            role.short_profile_name(&conf),
            "test_462440-assumed-role-test-role-mfa-user_short-term"
        );
    }
}
