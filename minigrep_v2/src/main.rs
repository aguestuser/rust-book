extern crate minigrep_v2;
use minigrep_v2::Config;

use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Could not parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_v2::run(cfg) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
