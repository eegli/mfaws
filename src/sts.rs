use crate::config::Config;
use crate::profile::LongTermProfile;
use async_trait::async_trait;
use aws_config::ConfigLoader;
use aws_credential_types::Credentials;
use aws_sdk_sts::{
    client::Client,
    error::{ProvideErrorMetadata, SdkError},
};

#[async_trait]
pub trait StsAction {
    type Output;
    async fn execute(
        &self,
        config: &Config,
        lt_profile: &LongTermProfile,
    ) -> Result<Self::Output, anyhow::Error>;

    fn get_mfa_token(&self) -> Result<String, anyhow::Error> {
        let mut input = String::new();
        println!("Enter MFA code:");
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_owned();
        Ok(input)
    }
}

impl LongTermProfile {
    pub async fn create_client(&self) -> Client {
        let credentials =
            Credentials::from_keys(self.access_key.clone(), self.secret_key.clone(), None);
        let conf = ConfigLoader::default()
            .credentials_provider(credentials)
            .region("us-east-1")
            .load()
            .await;
        Client::new(&conf)
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
