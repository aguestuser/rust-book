extern crate art_1978;

use art_1978::mix;
use art_1978::PrimaryColor;
use std::process;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
    process::exit(0);
}
