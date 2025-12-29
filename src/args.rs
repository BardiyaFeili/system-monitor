use clap::{Parser, ArgGroup};

#[derive(Parser, Debug)]
#[command(
    name = "system-monitor",
    about = "Minimal cross-platform system monitor",
    version
)]
#[command(
    group(
        ArgGroup::new("mode")
            .args(&["live", "log"])
            .multiple(false)
    )
)]
pub struct CliArgs {
    /// Live mode: updates output in place
    #[arg(long)]
    pub live: bool,

    /// Log mode: prints a new line every update
    #[arg(long)]
    pub log: bool,

    /// Update interval in seconds
    #[arg(long, default_value_t = 1)]
    pub interval: u64,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
