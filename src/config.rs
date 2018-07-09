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
    /// use minigrep::config::Config;
    ///
    /// let mut args: Vec<String> = Vec::new();
    /// args.push(String::from("the_binary"));
    /// args.push(String::from("the_query"));
    /// args.push(String::from("the_filename"));
    ///
    /// let parsed_args = Config::new(&args);
    /// assert_eq!(parsed_args.filename, "the_filename");
    /// ```
    pub fn new(args: &[String]) -> Config {
        // note that calls to clone are probably under-performant, but this is
        // such a simplistic domain, it really does not matter for now.
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
