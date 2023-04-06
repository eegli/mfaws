use crate::cmds::StsCommand;
use crate::config::Config;
use crate::creds::CredentialsHandler;
use crate::profile::{LongTermProfile, ShortTermProfile};
use async_trait::async_trait;
use aws_sdk_sts::error::{ProvideErrorMetadata, SdkError};

mod assume_role;
mod session_token;

#[async_trait]
pub trait StsAction {
    const DEFAULT_DURATION: i32;

    async fn get_profile(
        &self,
        config: &Config,
        lt_profile: &LongTermProfile,
    ) -> Result<ShortTermProfile, anyhow::Error>;

    fn get_mfa_token(&self) -> Result<String, anyhow::Error> {
        let mut input = String::new();
        println!("Enter MFA code:");
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_owned();
        Ok(input)
    }
}

impl StsCommand {
    fn get_profile_name(&self, config: &Config) -> String {
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

    pub async fn get_credentials(&self, config: &Config) -> anyhow::Result<()> {
        let mut creds_handler = CredentialsHandler::from_file(config.credentials.as_path())?;

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

        let st_profile = match self {
            StsCommand::AssumeRole(ref op) => {
                info!("Assuming role \"{}\" for \"{}\"", op.role_arn, op.role_name);
                op.get_profile(&config, &lt_profile).await?
            }
            StsCommand::GetSessionToken(ref op) => {
                info!("Getting session token");
                op.get_profile(&config, &lt_profile).await?
            }
        };

        creds_handler.set_short_term_profile(&st_profile, &st_profile_name);
        creds_handler
            .0
            .write_to_file(config.credentials.as_path())?;

        info!(
            "Successfully added short-term credentials \"{}\"",
            st_profile_name
        );

        Ok(())
    }
}

pub fn extract_sts_err<T>(err: SdkError<T>) -> anyhow::Error
where
    T: ProvideErrorMetadata,
{
    anyhow::anyhow!(err
        .meta()
        .message()
        .map(String::from)
        .unwrap_or(format!("Failed to get STS credentials: {}", { err })))
}

#[cfg(test)]
mod test {
    use crate::{
        cmds::{
            sts::{AssumeRole, SessionToken},
            StsCommand,
        },
        config::Config,
    };

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
