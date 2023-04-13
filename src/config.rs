use std::path::PathBuf;

#[derive(clap::Parser, Debug, Default)]
pub struct Config {
    #[arg(
        long,
        env = "AWS_SHARED_CREDENTIALS_FILE",
        global = true,
        default_value = ".aws/credentials",
        help = "Location of the AWS credentials file. Can be a relative path from your home directory or an absolute path to the file"
    )]
    pub credentials_path: PathBuf,
}

impl Config {
    pub fn init(&mut self) -> anyhow::Result<()> {
        self.validate_credentials_path()?;
        Ok(())
    }

    fn validate_credentials_path(&mut self) -> anyhow::Result<()> {
        if self.credentials_path.is_relative() {
            self.credentials_path = dirs::home_dir()
                .expect("Cannot find home directory")
                .join(self.credentials_path.as_path());
        }
        if !self.credentials_path.is_file() {
            anyhow::bail!("The credentials file does not exist");
        }
        Ok(())
    }
}
