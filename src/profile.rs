use std::{borrow::Cow, ops::Deref, str::FromStr, time::SystemTime};

use aws_config::ConfigLoader;
use aws_credential_types::Credentials;
use aws_sdk_sts::{
    primitives::DateTime as AwsDateTime, types::Credentials as StsCredentials, Client,
};
use aws_smithy_types::date_time::{ConversionError, DateTimeParseError, Format};

#[derive(Debug, Default)]
pub struct LongTermProfile<'a> {
    pub name: Cow<'a, str>,
    pub access_key: Cow<'a, str>,
    pub secret_key: Cow<'a, str>,
    pub mfa_device: Cow<'a, str>,
}
#[derive(Debug, Default)]
pub struct ShortTermProfile<'a> {
    pub access_key: String,
    pub secret_key: String,
    pub session_token: String,
    pub expiration: DateTime,
    pub assumed_role_id: Option<String>,
    pub assumed_role_arn: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone)]
pub struct DateTime(pub AwsDateTime);

pub trait Profile {
    const ACCESS_KEY: &'static str = "aws_access_key_id";
    const SECRET_KEY: &'static str = "aws_secret_access_key";
    const MFA_DEVICE: &'static str = "aws_mfa_device";
    const SESSION_TOKEN: &'static str = "aws_session_token";
    const ASSUMED_ROLE_ARN: &'static str = "assumed_role_arn";
    const ASSUMED_ROLE_ID: &'static str = "assumed_role_id";
    const EXPIRATION: &'static str = "expiration";
}

impl<'a> Profile for LongTermProfile<'a> {}
impl<'a> Profile for ShortTermProfile<'a> {}

impl Default for DateTime {
    fn default() -> Self {
        Self(AwsDateTime::from_secs(0))
    }
}

impl Deref for DateTime {
    type Target = AwsDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for DateTime {
    type Err = DateTimeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(AwsDateTime::from_str(s, Format::DateTime)?))
    }
}

impl TryFrom<DateTime> for SystemTime {
    type Error = ConversionError;
    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        Ok(value.0.try_into()?)
    }
}

impl<'a> LongTermProfile<'a> {
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

impl<'a> ShortTermProfile<'a> {
    pub fn format_expiration(&self) -> String {
        self.expiration.fmt(Format::DateTime).unwrap()
    }
}

impl<'a> TryFrom<Option<StsCredentials>> for ShortTermProfile<'a> {
    type Error = anyhow::Error;
    fn try_from(creds: Option<StsCredentials>) -> anyhow::Result<Self> {
        let creds = creds.ok_or_else(|| anyhow::anyhow!("Failed to extract STS credentials"))?;

        if let (Some(access_key), Some(secret_key), Some(session_token), Some(expiration)) = (
            creds.access_key_id,
            creds.secret_access_key,
            creds.session_token,
            creds.expiration,
        ) {
            Ok(Self {
                access_key,
                secret_key,
                session_token,
                expiration: DateTime(expiration),
                assumed_role_arn: None,
                assumed_role_id: None,
            })
        } else {
            anyhow::bail!("One or more invalid STS credentials")
        }
    }
}
