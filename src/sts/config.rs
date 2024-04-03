#[derive(clap::Args, Debug, Default)]
pub struct CommonStsConfig {
    #[arg(
        long = "profile",
        env = "AWS_PROFILE",
        default_value = "default",
        help = "The AWS credentials profile to use"
    )]
    pub profile_name: String,
    #[arg(long = "device", env = "MFA_DEVICE", help = "The MFA Device ARN")]
    pub mfa_device: Option<String>,
    #[arg(long, help = "The one-time password from your MFA device")]
    pub otp: Option<String>,
    #[arg(
        long,
        env = "MFA_DURATION",
        help = "The duration, in seconds, for which the temporary credentials should remain valid"
    )]
    pub duration: Option<i32>,
    #[arg(
        long = "short-term-suffix",
        default_value = "short-term",
        help = "To identify the auto-generated short-term credential profile"
    )]
    pub short_term_suffix: String,
    #[arg(
        long = "force",
        default_value = "false",
        help = "Force the creation of a new short-term profile even if one already exists"
    )]
    pub force_new_credentials: bool,
    #[arg(
        long,
        global = true,
        default_value = "us-east-1",
        help = "The STS region to use for the AWS client"
    )]
    pub sts_region: String,
}

impl CommonStsConfig {
    pub fn init(&mut self) -> anyhow::Result<()> {
        self.validate_profile_name()?;
        Ok(())
    }

    fn validate_profile_name(&self) -> anyhow::Result<()> {
        if self.profile_name.ends_with(&self.short_term_suffix) {
            anyhow::bail!("Profile name cannot end with the short-term suffix");
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn validate_profile_name() {
        let mut config = CommonStsConfig {
            profile_name: "default".to_string(),
            short_term_suffix: "short-term".to_string(),
            ..Default::default()
        };

        assert!(config.validate_profile_name().is_ok());

        config.profile_name = "xshort-term".to_string();
        assert!(config.validate_profile_name().is_err());
    }
}
