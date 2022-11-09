use std::{env, process};

use nav_sftp::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem while parsing command line arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = nav_sftp::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
