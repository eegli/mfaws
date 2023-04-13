use async_trait::async_trait;
use aws_sdk_sts::error::{ProvideErrorMetadata, SdkError};

use crate::{
    creds::CredentialsHandler,
    profile::{LongTermProfile, ShortTermProfile},
    sts::config::CommonStsConfig,
};

pub mod assume_role;
pub mod config;
pub mod session_token;

#[async_trait]
pub trait ShortTermCredentials {
    const DEFAULT_DURATION: i32;

    async fn get_credentials(
        &self,
        config: &CommonStsConfig,
        mfa_token: String,
        lt_profile: &LongTermProfile,
    ) -> anyhow::Result<ShortTermProfile>;

    fn short_profile_name(&self) -> String;
    fn config<'c>(&'c self) -> &'c CommonStsConfig;
}

pub fn get_mfa_token() -> anyhow::Result<String> {
    let mut input = String::new();
    println!("Enter MFA code:");
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_owned();
    Ok(input)
}

pub async fn get_st_profile<T>(cmd: T, mut handler: CredentialsHandler) -> anyhow::Result<()>
where
    T: ShortTermCredentials,
{
    let config = cmd.config();
    let lt_profile = handler.get_long_term_profile(cmd.config())?;

    info!("Using long-term profile \"{}\"", lt_profile.name);

    let st_profile_name = cmd.short_profile_name();

    if let Some(remaining_time) = handler.is_profile_still_valid(&st_profile_name) {
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
    let st_profile = cmd.get_credentials(&config, mfa_token, &lt_profile).await?;

    handler.set_short_term_profile(&st_profile, &st_profile_name);
    handler.to_file()?;

    info!(
        "Successfully added short-term credentials \"{}\"",
        st_profile_name
    );

    Ok(())
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
