use clap::{arg, command, Parser};
use log::{error, info};
use tokio::sync::mpsc;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() {
    simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default()).unwrap();
    info!("Hello, world!");
}
