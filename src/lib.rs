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
    /// In this case, let's presume that the input args are coming from `env::args().collect()`.
    ///
    /// The constructor will take any reference to a `Vec<String>`.
    /// ```
    /// use minigrep::Config;
    ///
    /// # let mut args: Vec<String> = vec!["minigrep".to_string(), "q".to_string(), "file.txt".to_string()];
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

/// runs the given config and returns the results of the query
pub fn exec(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let (results, line_num) = search(config.query, contents);
    for line in results {
        println!("Line {}: '{}'", line_num.to_string(), line);
    }
    Ok(())
}

/// executes a query to locate the desired line in the data
fn search(query: String, contents: String) -> (Vec<String>, usize) {
    let mut results = Vec::new();
    let mut line_num: usize = 0;
    let mut i = 0;
    for line in contents.as_str().lines() {
        i += 1;
        if line.contains(&query) {
            results.push(line.to_string());
            line_num = i;
        }
    }
    (results, line_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "red".to_string();
        let contents = r#"
There is
a red house
over yonder"#.to_string();
        let (result, _) = search(query, contents);
        assert_eq!(vec!["a red house"], result);
    }
}
