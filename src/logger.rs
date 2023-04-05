pub use simplelog::*;

pub fn init() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        ConfigBuilder::new().add_filter_allow_str("mfaws").build(), // suppress logging from other crates
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
