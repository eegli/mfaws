use crate::config::Config;
use crate::profile::{LongTermProfile, ShortTermProfile};
use async_trait::async_trait;
use aws_sdk_sts::error::{ProvideErrorMetadata, SdkError};

mod assume_role;
mod session_token;

#[async_trait]
pub trait StsCredentialsRequest {
    const DEFAULT_DURATION: i32;

    async fn get_credentials(
        &self,
        config: &Config,
        mfa_token: String,
        lt_profile: &LongTermProfile,
    ) -> anyhow::Result<ShortTermProfile>;
}

pub fn get_mfa_token() -> anyhow::Result<String> {
    let mut input = String::new();
    println!("Enter MFA code:");
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_owned();
    Ok(input)
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
