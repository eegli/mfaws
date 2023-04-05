use crate::{
    config::Config,
    profile::{DateTime, LongTermProfile, Profile, ShortTermProfile},
    utils::get_remaining_time,
};
use ini::{Ini, Properties};
use std::path::Path;
use std::{fmt::Debug, time::SystemTime};
use thiserror::Error;
pub struct CredentialsHandler(pub Ini);

impl CredentialsHandler {
    pub fn _new(buf: &str) -> Result<Self, ini::ParseError> {
        Ok(Self(Ini::load_from_str(buf)?))
    }

    pub fn from_file<P>(path: P) -> Result<Self, ini::Error>
    where
        P: AsRef<Path>,
    {
        Ok(Self(Ini::load_from_file(path)?))
    }

    pub fn get_long_term_profile(
        &self,
        conf: &Config,
    ) -> Result<LongTermProfile, CredentialsError> {
        let profile = &conf.profile_name;
        let sections = self
            .0
            .section_all(Some(profile))
            .take(2)
            .collect::<Vec<_>>();

        match sections.len() {
            0 => Err(CredentialsError::ProfileNotFound(profile.to_owned())),
            1 => {
                let section = sections[0];
                let mut pf = LongTermProfile {
                    name: profile.to_owned(),
                    ..Default::default()
                };
                match section.get(LongTermProfile::ACCESS_KEY) {
                    Some(access_key) => pf.access_key = access_key.to_owned(),
                    None => Err(CredentialsError::NoAccessKey(profile.to_owned()))?,
                }
                match section.get(LongTermProfile::SECRET_KEY) {
                    Some(secret_key) => pf.secret_key = secret_key.to_owned(),
                    None => Err(CredentialsError::NoSecretKey(profile.to_owned()))?,
                }
                match conf
                    .mfa_device
                    .as_deref()
                    .or(section.get(LongTermProfile::MFA_DEVICE))
                {
                    Some(mfa_device) => pf.mfa_device = mfa_device.to_owned(),
                    None => Err(CredentialsError::NoMfaDevice(profile.to_owned()))?,
                }

                Ok(pf)
            }
            _ => Err(CredentialsError::MultipleProfilesFound(profile.to_owned())),
        }
    }

    pub fn get_profile(&self, profile_name: &str) -> Option<&Properties> {
        self.0.section(Some(profile_name))
    }

    pub fn is_profile_still_valid(&self, profile_name: &str) -> Option<String> {
        self.get_profile(profile_name)
            .and_then(|section| section.get(LongTermProfile::EXPIRATION))
            .and_then(|s| s.parse::<DateTime>().ok())
            .and_then(|s| SystemTime::try_from(s).ok())
            .and_then(get_remaining_time)
    }

    pub fn set_short_term_profile(&mut self, profile: &ShortTermProfile, profile_name: &str) {
        if let Some(ref arn) = &profile.assumed_role_arn {
            self.0.set_to(
                Some(profile_name),
                LongTermProfile::ASSUMED_ROLE_ARN.to_owned(),
                arn.to_owned(),
            );
        }

        if let Some(ref id) = &profile.assumed_role_id {
            self.0.set_to(
                Some(profile_name),
                LongTermProfile::ASSUMED_ROLE_ID.to_owned(),
                id.to_owned(),
            );
        }

        self.0
            .with_section(Some(profile_name))
            .set(LongTermProfile::EXPIRATION, profile.format_expiration())
            .set(LongTermProfile::ACCESS_KEY, profile.access_key.to_owned())
            .set(LongTermProfile::SECRET_KEY, profile.secret_key.to_owned())
            .set(
                LongTermProfile::SESSION_TOKEN,
                profile.session_token.to_owned(),
            );
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum CredentialsError {
    #[error("No access key found for \"{0}\"")]
    NoAccessKey(String),
    #[error("No secret key found for \"{0}\"")]
    NoSecretKey(String),
    #[error("No MFA device found for \"{0}\"")]
    NoMfaDevice(String),
    #[error("Profile \"{0}\" not found")]
    ProfileNotFound(String),
    #[error("Multiple profiles found for \"{0}\"")]
    MultipleProfilesFound(String),
}

#[cfg(test)]
mod test_long_term_profile {

    use super::*;

    #[test]
    fn err_no_access_key() {
        let ini = r#"[test]
        aws_secret_access_key = 1
        aws_mfa_device = 2"#;
        let handler = CredentialsHandler::_new(ini).unwrap();
        let config = Config {
            profile_name: "test".to_owned(),
            ..Default::default()
        };
        assert!(matches!(
            handler.get_long_term_profile(&config).unwrap_err(),
            CredentialsError::NoAccessKey(_)
        ));
    }

    #[test]
    fn err_no_secret_key() {
        let ini = r#"[test]
        aws_access_key_id = 1
        aws_mfa_device = 2"#;
        let handler = CredentialsHandler::_new(ini).unwrap();
        let config = Config {
            profile_name: "test".to_owned(),
            ..Default::default()
        };
        assert!(matches!(
            handler.get_long_term_profile(&config).unwrap_err(),
            CredentialsError::NoSecretKey(_)
        ));
    }

    #[test]
    fn err_no_mfa_device() {
        let ini = r#"[test]
        aws_access_key_id = 1
        aws_secret_access_key = 1"#;
        let handler = CredentialsHandler::_new(ini).unwrap();
        let config = Config {
            profile_name: "test".to_owned(),
            ..Default::default()
        };
        assert!(matches!(
            handler.get_long_term_profile(&config).unwrap_err(),
            CredentialsError::NoMfaDevice(_)
        ));
    }

    #[test]
    fn err_no_profile() {
        let ini = "";
        let handler = CredentialsHandler::_new(ini).unwrap();
        let config = Config {
            profile_name: "test".to_owned(),
            ..Default::default()
        };
        assert!(matches!(
            handler.get_long_term_profile(&config).unwrap_err(),
            CredentialsError::ProfileNotFound(_)
        ));
    }
    #[test]
    fn err_multiple_profiles() {
        let ini = r#"[test]
        bla = 2
        [test]
        bla = 3"#;
        let handler = CredentialsHandler::_new(ini).unwrap();
        let config = Config {
            profile_name: "test".to_owned(),
            ..Default::default()
        };
        assert!(matches!(
            handler.get_long_term_profile(&config).unwrap_err(),
            CredentialsError::MultipleProfilesFound(_)
        ));
    }

    #[test]
    fn converts_successfully_1() {
        let ini = r#"[test]
        aws_access_key_id = 1
        aws_secret_access_key = 1
        aws_mfa_device = 2"#;
        let handler = CredentialsHandler::_new(ini).unwrap();
        let config = Config {
            profile_name: "test".to_owned(),
            ..Default::default()
        };
        assert_eq!(handler.get_long_term_profile(&config).unwrap().name, "test");
    }

    #[test]
    fn converts_successfully_2() {
        let ini = r#"[test]
        aws_access_key_id = 1
        aws_secret_access_key = 1"#;
        let handler = CredentialsHandler::_new(ini).unwrap();
        let config = Config {
            profile_name: "test".to_owned(),
            mfa_device: Some("device".to_owned()),
            ..Default::default()
        };
        assert!(handler.get_long_term_profile(&config).is_ok());
    }
}

#[cfg(test)]
mod test_short_term_profile {

    use crate::profile::Profile;

    use super::*;

    #[test]
    fn writes_st_profile_with_assumed_role() {
        let mut handler = CredentialsHandler::_new("").unwrap();
        let profile = ShortTermProfile {
            assumed_role_id: Some("id".to_owned()),
            assumed_role_arn: Some("arn".to_owned()),
            ..Default::default()
        };
        let profile_name = "test";
        handler.set_short_term_profile(&profile, profile_name);
        let section = handler.0.section(Some(profile_name)).unwrap();

        assert!(section.contains_key(ShortTermProfile::ASSUMED_ROLE_ARN));
        assert!(section.contains_key("assumed_role_id"));
        assert!(section.contains_key("expiration"));
        assert!(section.contains_key("aws_access_key_id"));
        assert!(section.contains_key("aws_secret_access_key"));
        assert!(section.contains_key("aws_session_token"));
    }

    #[test]
    fn writes_st_profile_without_assumed_role() {
        let mut handler = CredentialsHandler::_new("").unwrap();
        let profile = ShortTermProfile {
            assumed_role_id: None,
            assumed_role_arn: None,
            ..Default::default()
        };
        let profile_name = "test";
        handler.set_short_term_profile(&profile, profile_name);
        let section = handler.0.section(Some(profile_name)).unwrap();
        assert!(!section.contains_key(ShortTermProfile::ASSUMED_ROLE_ARN));
        assert!(!section.contains_key("assumed_role_id"));
        assert!(section.contains_key("expiration"));
        assert!(section.contains_key("aws_access_key_id"));
        assert!(section.contains_key("aws_secret_access_key"));
        assert!(section.contains_key("aws_session_token"));
    }
}
