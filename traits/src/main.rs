use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    ////////////
    // traits //
    ////////////

    // a trait (abstract)
    pub trait Summary {
        // no default
        // fn summarize(&self) -> String;

        // with default
        fn summarize(&self) -> String {
            String::from("Read more...")
        }

        // we could also provide default impl.'s of some but not all methods
    }

    // types that implement trait (concrete)
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {} // uses defaults

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    //////////////////
    // trait bounds //
    //////////////////

    // basics

    // specifies that T must implement Summary trait
    fn foo<T: Summary>(t: T) {}

    // we can specify multiple trait bounds on a type parameter:
    fn bar<T: Summary + Clone>(t: T) {}

    // this can get messy! so we might want to rewrite this...
    fn some_func_cluttered<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}

    // as this...
    fn some_func_clean<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        42
    }

    // see impl. of `largest` in `generics/main.rs` for great example of trait bounds at works!

    // conditional implementations:

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        // implement `new` for all Pair<T>
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        // only implement for Pair<T> where T satisfies type bounds
        // ie: if T implements Display & PartialOrd
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest number is x = {}", self.x);
            } else {
                println!("The largest number is y = {}", self.y);
            }
        }
    }

    // blanket implementations:
    // from std lib:
    // impl<T: Display> ToString for T {
    //     // snip
    // }
    // implements ToString on any type T that implements Display
}
