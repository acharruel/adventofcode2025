use anyhow::Result;
use clap::Parser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

#[derive(Debug, Parser)]
struct Cli {
    /// log level
    #[arg(long = "log")]
    #[arg(env = "AOC_LOG")]
    #[arg(default_value = "info")]
    pub log_level: String,

    #[clap(short, long)]
    /// Index of the day
    day: u8,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    setup_logging(&args.log_level)?;
    adventofcode2025::run(args.day)?;
    Ok(())
}

fn setup_logging(log_level: &str) -> Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::OFF.into())
        .from_env()?
        .add_directive(log_level.parse()?);
    let layer = tracing_subscriber::fmt::layer().without_time();
    let tracer = tracing_subscriber::registry().with(layer).with(filter);
    tracing::subscriber::set_global_default(tracer)?;
    Ok(())
}
