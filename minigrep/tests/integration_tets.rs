extern crate minigrep;
use minigrep::Config;

#[test]
fn performs_case_sensitive_search() {
    assert_eq!(
        minigrep::run(Config {
            query: String::from("to"),
            path: String::from("poem.txt"),
            case_sensitive: true,
        }).unwrap(),
        vec![
            String::from("Are you nobody, too?"),
            String::from("How dreary to be somebody!"),
        ]
    )
}

#[test]
fn performs_case_insensitive_search() {
    assert_eq!(
        minigrep::run(Config {
            query: String::from("to"),
            path: String::from("poem.txt"),
            case_sensitive: false,
        }).unwrap(),
        vec![
            String::from("Are you nobody, too?"),
            String::from("How dreary to be somebody!"),
            String::from("To tell your name the livelong day"),
            String::from("To an admiring bog!"),
        ]
    )
}
