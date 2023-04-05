use crate::config::Config;
use aws_sdk_sts::primitives::DateTime as AwsDateTime;
use aws_sdk_sts::types::Credentials as StsCredentials;
use aws_smithy_types::date_time::{ConversionError, DateTimeParseError, Format};
use std::{ops::Deref, str::FromStr, time::SystemTime};

#[derive(Debug, Default)]
pub struct LongTermProfile {
    pub name: String,
    pub access_key: String,
    pub secret_key: String,
    pub mfa_device: String,
}
#[derive(Debug, Default)]
pub struct ShortTermProfile {
    pub access_key: String,
    pub secret_key: String,
    pub session_token: String,
    pub expiration: DateTime,
    pub assumed_role_id: Option<String>,
    pub assumed_role_arn: Option<String>,
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

impl Profile for LongTermProfile {}
impl Profile for ShortTermProfile {}

pub trait ProfileName {
    fn short_profile_name(&self, config: &Config) -> String;
}

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

impl ShortTermProfile {
    pub fn format_expiration(&self) -> String {
        self.expiration.fmt(Format::DateTime).unwrap()
    }
}

impl TryFrom<Option<&StsCredentials>> for ShortTermProfile {
    type Error = anyhow::Error;
    fn try_from(creds: Option<&StsCredentials>) -> anyhow::Result<Self> {
        let creds = creds.ok_or_else(|| anyhow::anyhow!("Failed to extract STS credentials"))?;

        if let (
            Some(access_key_id),
            Some(secret_access_key),
            Some(session_token),
            Some(expiration),
        ) = (
            creds.access_key_id(),
            creds.secret_access_key(),
            creds.session_token(),
            creds.expiration(),
        ) {
            Ok(Self {
                access_key: access_key_id.to_owned(),
                secret_key: secret_access_key.to_owned(),
                session_token: session_token.to_owned(),
                expiration: DateTime(expiration.to_owned()),
                assumed_role_arn: None,
                assumed_role_id: None,
            })
        } else {
            anyhow::bail!("One or more invalid STS credentials")
        }
    }
}
