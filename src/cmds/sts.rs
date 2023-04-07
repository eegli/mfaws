use super::{Command, StsCommand};
use crate::{
    config::Config,
    sts::{get_mfa_token, StsCredentialsRequest},
};
use async_trait::async_trait;

#[derive(clap::Args, Debug, Default)]
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

#[derive(clap::Args, Debug, Default)]
pub struct SessionToken;

#[async_trait]
impl Command for StsCommand {
    async fn exec(&self, config: &Config) -> anyhow::Result<()> {
        let mut creds_handler = config.credentials_handler()?;

        let lt_profile = creds_handler.get_long_term_profile(&config)?;

        info!("Using long-term profile \"{}\"", lt_profile.name);

        let st_profile_name = self.get_profile_name(config);

        if let Some(remaining_time) = creds_handler.is_profile_still_valid(&st_profile_name) {
            match config.force_new_credentials {
                false => {
                    info!(
                        "Found existing short-term profile \"{}\" that is valid for the next {}",
                        st_profile_name, remaining_time
                    );
                    return Ok(());
                }
                true => info!(
                    "Discarding existing short-term profile \"{}\" (--force was used)",
                    st_profile_name,
                ),
            }
        };
        let mfa_token = get_mfa_token()?;
        let st_profile = match self {
            StsCommand::AssumeRole(ref op) => {
                info!("Assuming role \"{}\" for \"{}\"", op.role_arn, op.role_name);
                op.get_credentials(&config, mfa_token, &lt_profile).await?
            }
            StsCommand::GetSessionToken(ref op) => {
                info!("Getting session token");
                op.get_credentials(&config, mfa_token, &lt_profile).await?
            }
        };

        creds_handler.set_short_term_profile(&st_profile, &st_profile_name);
        creds_handler
            .0
            .write_to_file(config.credentials_path.as_path())?;

        info!(
            "Successfully added short-term credentials \"{}\"",
            st_profile_name
        );

        Ok(())
    }
}

impl StsCommand {
    pub fn get_profile_name(&self, config: &Config) -> String {
        match self {
            StsCommand::AssumeRole(ref role) => {
                let arn = role
                    .role_arn
                    .split([':', '/'])
                    .skip(4)
                    .collect::<Vec<&str>>()
                    .join("-");
                config.profile_name.clone()
                    + "_"
                    + &arn
                    + "-"
                    + &role.role_name
                    + "_"
                    + &config.short_term_suffix
            }

            StsCommand::GetSessionToken(_) => {
                format!("{}-{}", config.profile_name, config.short_term_suffix)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn session_token_st_profile_name() {
        let cmd = StsCommand::GetSessionToken(SessionToken);
        let conf = Config {
            profile_name: "test".to_string(),
            short_term_suffix: "short-term".to_string(),
            ..Default::default()
        };
        assert_eq!(cmd.get_profile_name(&conf), "test-short-term");
    }
    #[test]
    fn assume_role_st_profile_name() {
        let cmd = StsCommand::AssumeRole(AssumeRole {
            role_arn: "arn:aws:sts::462440:assumed-role/test-role".to_string(),
            role_name: "mfa-user".to_string(),
            ..Default::default()
        });
        let conf = Config {
            profile_name: "test".to_string(),
            short_term_suffix: "short-term".to_string(),
            ..Default::default()
        };
        assert_eq!(
            cmd.get_profile_name(&conf),
            "test_462440-assumed-role-test-role-mfa-user_short-term"
        );
    }
}
