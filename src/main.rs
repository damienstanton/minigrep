use std::env;
use std::fs::File;
use std::io::prelude::*;

mod config;
use config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("All arguments: {:?}", &args[1..]);
    let config = Config::new(&args);
    println!("Query is: {:?}", config.query);
    println!("Filename is: {:?}", config.filename);

    let mut f = File::open(config.filename).expect("unable to open file");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("unable to read content from file");

    println!("Contents of file: \n{}", contents)
}
