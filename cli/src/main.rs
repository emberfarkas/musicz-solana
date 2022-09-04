mod error;
mod funcational;
mod logger;

use clap::{Arg, Command};
use log::error;


fn main() {
    simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default()).unwrap();

    let m = Command::new("cli")
        .author("joseph@google.com")
        .about("Explains in brief what the program does")
        .version("1.0.2")
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .arg(Arg::new("debug").short('d').help("truning on debug mod"))
        .get_matches();

    // if let Some(c) = m.value_of("d") {
    //     println!("command {}", c)
    // }

    log::info!("cli begin:");

    if let Err(e) = crate::funcational::get_account() {
        error!("{}", e);
    }
    
}
