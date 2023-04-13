use std::path::PathBuf;

#[derive(clap::Parser, Debug, Default)]
pub struct Config {
    #[arg(
        long,
        env = "AWS_SHARED_CREDENTIALS_FILE",
        global = true,
        value_parser = valid_credentials_path,
        default_value = ".aws/credentials",
        help = "Location of the AWS credentials file. Can be a relative path from your home directory or an absolute path to the file"
    )]
    pub credentials_path: PathBuf,
}

fn valid_credentials_path(s: &str) -> Result<PathBuf, String> {
    let mut path = PathBuf::from(s);
    if path.is_relative() {
        path = dirs::home_dir()
            .ok_or_else(|| format!("Cannot find home directory"))?
            .join(path.as_path());
    }
    if !path.is_file() {
        return Err(format!("Not a valid credentials file",));
    }
    Ok(path)
}
