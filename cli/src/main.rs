mod client;
mod error;
mod funcational;
mod logger;
mod scan;

use clap::{Arg, Command, arg};
use log::{error, info};


fn main() {
    simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default()).unwrap();

    let m = Command::new("cli")
        .author("joseph@google.com")
        .about("Explains in brief what the program does")
        .version("0.0.1")
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .arg(Arg::new("verbose").long("verbose").short('v').help("verbose detail output"))
        // .arg(arg!(-v --verbose <VERBOSE> "verbose detail output"))
        .subcommand(Command::new("account").about("get account").arg(Arg::new("address").long("address").short('a').help("eth address")))
        .subcommand(Command::new("db").about("test level db").arg(Arg::new("path").long("path").short('p').help("the path of db")))
        .subcommand(Command::new("scan").about("scan db"))
        .get_matches();

    let debug: bool = m.value_of_t("VEBOSE").unwrap_or_default();
    if debug {
        info!("debug mode");
    }
    
    info!("cli begin:");

    if let Some(sub_matches) = m.subcommand_matches("account") {
        let address : String = sub_matches.value_of_t("address").unwrap();
        if let Err(e) = crate::funcational::get_account(address) {
            error!("{}", e);
        }
    } else if let Some(sub_matches) = m.subcommand_matches("db") {
        let path : String = sub_matches.value_of_t("path").unwrap();
        if let Err(e) = crate::funcational::load_db(path) {
            error!("{}", e);
        }
    } else if let Some(sub_matches) = m.subcommand_matches("scan") {
        if let Err(e) = crate::funcational::scan() {
            error!("{}", e);
        }
    }
}
