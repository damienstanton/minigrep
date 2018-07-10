extern crate minigrep;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("All arguments: {:?}", &args[1..]);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Unable to parse arguments: {}", err);
        process::exit(1);
    });

    println!("Query is: {:?}", config.query);
    println!("Filename is: {:?}", config.filename);
    if let Err(err) = minigrep::run(config) {
        println!("Error: {}", err);
        process::exit(1);
    }
}
