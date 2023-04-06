#[derive(clap::Parser, Debug, Default)]
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

#[derive(clap::Parser, Debug, Default)]
pub struct SessionToken;
