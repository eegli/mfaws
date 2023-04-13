#[derive(clap::Args, Debug, Default)]
pub struct Clean {
    #[clap(long, default_value = "false")]
    pub all: Option<bool>,
}
