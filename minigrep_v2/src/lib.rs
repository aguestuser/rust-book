use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string provided"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename provided"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            path,
            case_sensitive,
        })
    }
}

pub fn run(cfg: Config) -> Result<Vec<String>, Box<Error>> {
    let contents = read_contents(cfg.path)?;
    // TODO: clean this up with lamdas! (ch 13)
    let results = if cfg.case_sensitive {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitive(&cfg.query, &contents)
    };
    for line in results.iter() {
        println!("{}", line);
    }
    Ok(results
        .iter()
        .map(|_str| _str.to_string())
        .collect::<Vec<String>>())
}

fn read_contents(path: String) -> Result<String, Box<Error>> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().as_str().contains(query.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: how to run these tests with `env::Args`
    // internet doesn't seem to know:
    // https://stackoverflow.com/questions/47441279/creating-an-stdenvargs-iterator-for-testing
    // ^--- solutions at top don't work, solution in bottom dodges the question

    // #[test]
    // fn parses_configs_from_valid_args() {
    //     assert_eq!(
    //         Ok(Config {
    //             query: "body".to_string(),
    //             path: "poem.txt".to_string(),
    //             case_sensitive: true,
    //         }),
    //         Config::new(std::env::Args([
    //             "path/to/binary".to_string(),
    //             "body".to_string(),
    //             "poem.txt".to_string()
    //         ]))
    //     )
    // }

    // #[test]
    // fn returns_err_from_invalid_args() {
    //     assert_eq!(
    //         Err("foo"),
    //         Config::new(&["body".to_string(), "poem.txt".to_string()])
    //     )
    // }

    #[test]
    fn reads_contents_from_file() {
        assert_eq!(
            read_contents("poem.txt".to_string()).unwrap(),
            "\
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
"
        )
    }

    #[test]
    fn performs_case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
";
        assert_eq!(search(query, contents), vec!["safe, fast, productive."])
    }

    #[test]
    fn performs_case_insensitive_search() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";
        assert_eq!(
            search_case_insensitive(query, contents),
            vec!["Rust:", "Trust me."]
        )
    }

}
