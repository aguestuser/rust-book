use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

const CONFIG_ARGS_ERR_MSG: &'static str = "not enough arguments (2 required)";

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err(CONFIG_ARGS_ERR_MSG);
        }
        Ok(Self {
            query: args[1].clone(),
            path: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
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
    // TODO: FP this ish! (coming in ch 13)
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }

    res
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line)
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_configs_from_valid_args() {
        assert_eq!(
            Ok(Config {
                query: "body".to_string(),
                path: "poem.txt".to_string(),
                case_sensitive: true,
            }),
            Config::new(&[
                "path/to/binary".to_string(),
                "body".to_string(),
                "poem.txt".to_string()
            ])
        )
    }

    #[test]
    fn returns_err_from_invalid_args() {
        assert_eq!(
            Err(CONFIG_ARGS_ERR_MSG),
            Config::new(&["body".to_string(), "poem.txt".to_string()])
        )
    }

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
