use minigrep::Config;
use std::{env, process};

/// cargo run -- search-string example-filename.txt
fn main() {
    let args= env::args();
    // dbg!(&args);

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
