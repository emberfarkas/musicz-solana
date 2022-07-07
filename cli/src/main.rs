use clap::{
    App, AppSettings, Arg, Command,
    ArgMatches, SubCommand,
};
use std::thread;
use std::time::Duration;
use rocksdb::{DB, Writable};

fn main() {
    // solana_logger::setup_with_default("solana=info");

    let m = Command::new("My Program")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Explains in brief what the program does")
        .arg(Arg::new("in_file"))
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .get_matches();
    
    let mut db = DB::open_default("/path/for/rocksdb/storage").unwrap();
    db.put(b"my key", b"my value");
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    db.delete(b"my key");

    // let _ = match m.subcommand() {
    //     ("create") => {}
    // };
    
    // let x = 3;
    // let y = x;
    // println!("{}", x);

    // let a: String = String::from("hello, world!");
    // let b = a;
    // println!("{}", a);
}
