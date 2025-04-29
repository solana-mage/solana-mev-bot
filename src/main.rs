mod bot;
mod config;
mod constants;
mod dex;
mod pools;
mod refresh;
mod transaction;

use clap::{App, Arg};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    info!("Starting Bot...");

    let matches = App::new("Solana MEV Bot")
        .version("1.0.0")
        .author("XXXXXXX")
        .about("Simple Solana MEV Bot")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .default_value("config.toml"),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap();
    info!("Using config file: {}", config_path);

    bot::run_bot(config_path).await?;

    Ok(())
}
