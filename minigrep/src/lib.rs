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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let commnad_line_option_provided = args.len() >= 4
                                           && (
                                               args[3..].contains(&String::from("-i"))
                                               || args[3..].contains(&String::from("--case-insensitive"))
                                           );

        let case_sensitive = if commnad_line_option_provided {
            false
        } else {
            env::var("CASE_INSENSITIVE").is_err()
        };

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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const POEM_FILE : &str = "resources/tests/poem.txt";

    #[test]
    fn not_enough_arguments() {
        assert_eq!(Config::new(&vec![]), Err("not enough arguments"));
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

        assert_eq!(Config::new(&args), Ok(expected_config));
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

        assert_eq!(Config::new(&args), Ok(expected_config));
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

        assert_eq!(Config::new(&args), Ok(expected_config));
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
