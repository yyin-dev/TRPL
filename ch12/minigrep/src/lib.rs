use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String, 
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Returning Result, instead of panicking, allows `main` to handle 
        // the Result, and exit the process more cleanly
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
    }
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> is a trait object, which means that the type will
    // implement the Error trait, but not specified to be any particular type.
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: & str, contents: &'a str) -> Vec<&'a str> {
    // The lifetime indicates that the returned vectro should contain string
    // slices that reference slices of the argument `contents`, not `query`.
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)            
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: & str, contents: &'a str) -> Vec<&'a str> {
    // The lifetime indicates that the returned vectro should contain string
    // slices that reference slices of the argument `contents`, not `query`.
    let query = query.to_lowercase();    
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)            
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.
";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}