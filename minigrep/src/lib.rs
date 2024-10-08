use std::env;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn multiple_results() {
        let query = "you";
        let contents = "\
Never gonna give you up,
Never gonna let you down,
Never gonna run around and,
Desert you.
";

        assert_eq!(
            vec![
                "Never gonna give you up,",
                "Never gonna let you down,",
                "Desert you."
            ],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive_multiple_results() {
        let query = "YoU";
        let contents = "\
Never gonna give you up,
Never gonna let you down,
Never gonna run around and,
Desert you.
";

        assert_eq!(
            vec![
                "Never gonna give you up,",
                "Never gonna let you down,",
                "Desert you."
            ],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive_one_result() {
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // NOTE: to_lowercase() takes our &str and creates a String with it
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // NOTE: Because how the to_lowercase() works, we have to borrow the
        // query because contains() expects a string reference.
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct Config<'a> {
    pub query: &'a str,
    pub path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("You have to enter at least 2 arguments!");
        }

        let query = &args[1];
        let path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
