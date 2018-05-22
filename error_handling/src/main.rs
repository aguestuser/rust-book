use std::fs::File;
use std::io::{self, BufRead, Read};

fn main() {
    ///////////////
    // panicking //
    ///////////////

    // boom...
    // panic!("crash and burn!");

    // run this with RUST_BACKTRACE=1 to see backtrace...
    // let v = vec![1, 2];
    // v[1000];

    ///////////////////////
    // returning results //
    ///////////////////////

    let f: Result<File, io::Error> = File::open("hello.txt");
    let msg = match f {
        Ok(file) => format!("File has contents: {:?}", io::BufReader::new(file).lines()),
        Err(e) => format!("Failed to open file: {:?}", e),
    };
    println!("{}", msg);

    ////////////////////////
    // propogating errors //
    ////////////////////////

    // this...
    fn read_name_from_file_verbose() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
        // ugh: like writing go! (also: so much stuttering in the pattern-matching!)
    }

    // is equivalent to this...
    fn read_name_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
        // much better!
        // `?` will :
        // - return unwrapped `Ok` value to lefhand on success
        // - return early with wrapped `Err` value on failure
        // NOTE: i smell MONADS under the hood!!! ;)
    }

    ////////////////////////////////////////////////////
    // enforcing safety guareantees w/ types & panics //
    ////////////////////////////////////////////////////

    // motivation: only allow user input btw/ 1 & 100 in guessing_game
    // but encode this constraint in type system so we don't have to check it over and over

    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 & 100. Got: {}", value);
            }
            Guess { value }
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }
}
