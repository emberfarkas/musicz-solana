mod client;
mod error;
mod funcational;
mod logger;
mod scan;
mod script_fun_demo;

use crate::funcational::get_account;
use clap::{arg, Command};
use log::{error, info};
use tokio::sync::mpsc;

fn cli() -> Command {
    Command::new("cli")
        .author("joseph@google.com")
        .about("Explains in brief what the program does")
        .version("0.0.1")
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        // .arg(arg!(-v --verbose <VERBOSE> "verbose detail output"))
        .subcommand(
            Command::new("account")
                .about("get account")
                .arg(arg!(-a --address <ADDRESS> "eth address")),
        )
        .subcommand(
            Command::new("db")
                .about("test level db")
                .arg(arg!(-p --path <PATH> "the path of db")),
        )
        .subcommand(Command::new("scan").about("scan db"))
        .subcommand(Command::new("eth").about("eth tx"))
}

#[tokio::main]
async fn main() {
    simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default()).unwrap();

    let m = cli().get_matches();
    match m.subcommand() {
        Some(("account", sub_matches)) => {
            let address = sub_matches.get_one::<String>("address").unwrap();
            if let Err(e) = get_account(address).await {
                error!("{}", e)
            }
        }
        Some(("db", sub_matches)) => {
            let path: &String = sub_matches.get_one::<String>("path").unwrap();
            if let Err(e) = crate::funcational::load_db(path).await {
                error!("{}", e);
            }
        }
        Some(("scan", sub_matches)) => {
            if let Err(e) = crate::funcational::scan().await {
                error!("{}", e);
            }
        }
        Some(("eth", sub_matches)) => {
            if let Err(e) = crate::funcational::eth_tx().await {
                error!("{}", e);
            }
        }
        _ => unreachable!(),
    }
}
