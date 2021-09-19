use std::{env, process};

use write_with_emoji::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = write_with_emoji::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
