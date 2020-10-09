use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)] // `Debug` and `PartialEq` traits are needed for this class to be used in `assert_eq!` macro that is a part of `not_enough_arguments` test
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = match args.next() {
            Some(arg) => {
                !(arg.contains(&String::from("-i")) || arg.contains(&String::from("--case-insensitive")))
            },
            None => true,
        };

        let case_sensitive = case_sensitive && env::var("CASE_INSENSITIVE").is_err();

        Ok(
            Config {
                query,
                filename,
                case_sensitive,
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const POEM_FILE : &str = "resources/tests/poem.txt";

    #[test]
    fn not_enough_arguments() {
        assert_eq!(Config::new(vec![String::from("dummy")].into_iter()), Err("Didn't get a query string"));
        assert_eq!(Config::new(vec![String::from("dummy"), String::from("dummy")].into_iter()), Err("Didn't get a file name"));
    }

    #[test]
    fn environment_dependent_config() {
        let args = vec![
            String::from("program-name"),
            String::from("frog"),
            String::from("poem.txt"),
        ];

        let expected_config = Config {
            query: String::from("frog"),
            filename: String::from("poem.txt"),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        };

        assert_eq!(Config::new(args.into_iter()), Ok(expected_config));
    }

    #[test]
    fn case_insensitive_config() {
        let args = vec![
            String::from("program-name"),
            String::from("frog"),
            String::from("poem.txt"),
            String::from("--case-insensitive"),
        ];

        let expected_config = Config {
            query: String::from("frog"),
            filename: String::from("poem.txt"),
            case_sensitive: false,
        };

        assert_eq!(Config::new(args.into_iter()), Ok(expected_config));
    }

    #[test]
    fn case_insensitive_config_short_form() {
        let args = vec![
            String::from("program-name"),
            String::from("frog"),
            String::from("poem.txt"),
            String::from("-i"),
        ];

        let expected_config = Config {
            query: String::from("frog"),
            filename: String::from("poem.txt"),
            case_sensitive: false,
        };

        assert_eq!(Config::new(args.into_iter()), Ok(expected_config));
    }

    #[test]
    fn no_results() {
        let query = "monomorphization";

        assert_eq!(
            Vec::new() as Vec<&str>,
            search(
                query,
                &fs::read_to_string(POEM_FILE).unwrap()
            )
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "The";

        assert_eq!(
            vec![
                "Then there’s a pair of us - don’t tell!",
                "They’d banish us, you know."
            ],
            search(
                query,
                &fs::read_to_string(POEM_FILE).unwrap()
            )
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "The";

        assert_eq!(
            vec![
                "Then there’s a pair of us - don’t tell!",
                "They’d banish us, you know.",
                "To tell your name the livelong day",
            ],
            search_case_insensitive(
                query,
                &fs::read_to_string(POEM_FILE).unwrap()
            )
        );
    }

    #[test]
    fn grep_test_file() -> Result<(), Box<dyn Error>> {
        let config = Config {
            query: String::from("frog"),
            filename: String::from(POEM_FILE),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        };

        run(config)
    }
}
