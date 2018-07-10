extern crate minigrep;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Unable to parse arguments: {}", err);
        process::exit(1);
    });

    println!("Finding '{}' in {}", config.query, config.filename);
    if let Err(err) = minigrep::exec(config) {
        println!("Error: {}", err);
        process::exit(1);
    }
}
