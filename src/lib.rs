use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

/// a container for configuration values
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /// reads in a vector of strings and returns a tuple of its elements
    /// # Example
    /// ```
    /// use minigrep::Config;
    ///
    /// let mut args: Vec<String> = vec!["minigrep".to_string(), "q".to_string(), "file.txt".to_string()];
    /// let parsed_args = &Config::new(&args).unwrap();
    /// assert_eq!(parsed_args.query, "q");
    /// assert_eq!(parsed_args.filename, "file.txt");
    /// ```
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // note that calls to clone are probably under-performant, but this is
        // such a simplistic domain, it really does not matter for now.
        if args.len() < 3 {
            return Err("Not enough arguments. Need at least a query to search for, and a file to search in.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

/// opens the given configuration and reads the data
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("unable to open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("Contents of file: \n{}", contents);
    Ok(())
}
