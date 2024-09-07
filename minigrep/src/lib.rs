use std::error::Error;
use std::fs;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

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
}

pub struct Config<'a> {
    pub query: &'a str,
    pub path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("You have to enter at least 2 arguments!");
        }

        let query = &args[1];
        let path = &args[2];

        Ok(Config { query, path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
