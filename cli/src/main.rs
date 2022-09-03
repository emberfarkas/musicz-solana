mod error;
mod funcational;

use clap::{Arg, Command};


fn main() {
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

    let ret = crate::funcational::get_account(); 
    match ret {
        Ok(_) => todo!(),
        Err(e)=> {
            println!("{}", e)
        }
    }
    
}
