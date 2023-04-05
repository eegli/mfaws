#[derive(clap::Parser, Debug, Default)]
pub struct Config {
    #[arg(
        long = "profile",
        env = "AWS_PROFILE",
        global = true,
        default_value = "default",
        help = "The AWS credentials profile to use."
    )]
    pub profile_name: String,
    #[arg(
        long = "device",
        env = "MFA_DEVICE",
        global = true,
        help = "The MFA Device ARN. This value can also be provided via the ~/.aws/credentials variable 'aws_mfa_device'."
    )]
    pub mfa_device: Option<String>,
    #[arg(
        long,
        env = "MFA_STS_DURATION",
        global = true,
        help = "The duration, in seconds, for which the temporary credentials should remain valid",
        long_help = "The duration, in seconds, for which the temporary credentials should remain valid. Defaults to 43200 (12 hours) for session tokens and 3600 (one hour) when assuming a role."
    )]
    pub duration: Option<i32>,
    #[arg(
        long = "short-term-suffix",
        global = true,
        default_value = "short-term",
        help = "To identify the auto-generated short term credential profile by [<profile_name>-SHORT_TERM_SUFFIX]."
    )]
    pub short_term_suffix: String,
}

impl Config {
    pub fn validate(self) -> anyhow::Result<Self> {
        if self.profile_name.ends_with(&self.short_term_suffix) {
            anyhow::bail!("The profile name cannot end with the short term suffix.");
        }
        Ok(self)
    }
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn test_config_validate() {
        let config = Config {
            profile_name: "default".to_string(),
            mfa_device: None,
            duration: None,
            short_term_suffix: "short-term".to_string(),
        };
        assert!(config.validate().is_ok());

        let config = Config {
            profile_name: "default-short-term".to_string(),
            mfa_device: None,
            duration: None,
            short_term_suffix: "short-term".to_string(),
        };
        assert!(config.validate().is_err());
    }
}
