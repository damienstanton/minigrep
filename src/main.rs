use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("All arguments: {:?}", &args[1..]);
    let (query, filename) = parse_args(&args);
    println!("Query is: {:?}", query);
    println!("Filename is: {:?}", filename);

    let mut f = File::open(filename).expect("unable to open file");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("unable to read content from file");

    println!("Contents of file: \n{}", contents)
}

/// parse_args reads in a vector of strings and returns a tuple of its elements
fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
