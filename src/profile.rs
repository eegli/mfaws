use aws_credential_types::Credentials as AWSCredentials;
use aws_sdk_sts::{
    config as StsConfig, primitives::DateTime as AWSDateTime, types as StsTypes,
    Client as STSClient,
};
use std::{borrow::Cow, ops::Deref, str::FromStr, time::SystemTime};

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
pub struct DateTime(pub AWSDateTime);

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
        Self(AWSDateTime::from_secs(0))
    }
}

impl Deref for DateTime {
    type Target = AWSDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for DateTime {
    type Err = aws_smithy_types::date_time::DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(AWSDateTime::from_str(
            s,
            aws_smithy_types::date_time::Format::DateTime,
        )?))
    }
}

impl TryFrom<DateTime> for SystemTime {
    type Error = aws_smithy_types::date_time::ConversionError;
    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        Ok(value.0.try_into()?)
    }
}

impl<'a> LongTermProfile<'a> {
    pub async fn create_client(&self, region: String) -> STSClient {
        let credentials = AWSCredentials::new(
            self.access_key.clone(),
            self.secret_key.clone(),
            None,
            None,
            "_",
        );
        let conf = StsConfig::Builder::new()
            .behavior_version(StsConfig::BehaviorVersion::v2024_03_28())
            .credentials_provider(credentials)
            .region(Some(StsConfig::Region::new(region)))
            .build();

        STSClient::from_conf(conf)
    }
}

impl<'a> ShortTermProfile<'a> {
    pub fn format_expiration(&self) -> String {
        self.expiration
            .fmt(aws_smithy_types::date_time::Format::DateTime)
            .unwrap()
    }
}

impl<'a> TryFrom<Option<StsTypes::Credentials>> for ShortTermProfile<'a> {
    type Error = anyhow::Error;
    fn try_from(creds: Option<StsTypes::Credentials>) -> anyhow::Result<Self> {
        let creds = creds.ok_or_else(|| anyhow::anyhow!("Failed to extract STS credentials"))?;

        Ok(Self {
            access_key: creds.access_key_id,
            secret_key: creds.secret_access_key,
            session_token: creds.session_token,
            expiration: DateTime(creds.expiration),
            assumed_role_arn: None,
            assumed_role_id: None,
        })
    }
}
