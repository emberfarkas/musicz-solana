mod client;
mod error;
mod funcational;
mod logger;
mod scan;

use std::string;

use clap::{arg, command, Arg, Command, Subcommand};
use log::{error, info};

fn cli() -> Command<'static> {
    Command::new("cli")
        .author("joseph@google.com")
        .about("Explains in brief what the program does")
        .version("0.0.1")
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .arg(arg!(-v --verbose <VERBOSE> "verbose detail output"))
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
}

fn main() {
    simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default()).unwrap();

    let m = cli().get_matches();

    if let Some(debug) = m.get_one::<bool>("VERBOSE") {
        info!("Value for -v: {}", debug);
    }

    info!("cli begin:");
    match m.subcommand() {
        Some(("account", sub_matches)) => {
            if let Some(c) = sub_matches.get_one::<String>("address") {
                println!("Value for -address: {}", c);
            }

            // if let Err(e) = crate::funcational::get_account(address) {
            //     error!("{}", e);
            // }
        }
        Some(("db", sub_matches)) => {
            let path: String = sub_matches.value_of_t("path").unwrap();
            if let Err(e) = crate::funcational::load_db(path) {
                error!("{}", e);
            }
        }
        Some(("scan", sub_matches)) => {
            if let Err(e) = crate::funcational::scan() {
                error!("{}", e);
            }
        }
        _ => unreachable!(),
    }
}
