use std::fmt::Display;

fn main() {
    ///////////////////////////
    // liftimes in functions //
    ///////////////////////////

    // defining a generic lifetime 'a:
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // works
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");
    let res = longest(s1.as_str(), s2.as_str());
    assert_eq!(res, "abcd");

    // also works
    // (s1 and s2 both share smaller lifetime of s2, which satisfies 'a)
    let s1 = String::from("abcd");
    {
        let s2 = String::from("xyz");
        let res = longest(s1.as_str(), s2.as_str());
        assert_eq!(res, "abcd");
    }

    // also works

    let s1;
    let s2;
    let res;
    {
        s1 = String::from("abcd");
        s2 = String::from("xyz");
        res = longest(s1.as_str(), s2.as_str());
    }
    assert_eq!(res, "abcd");

    // does not work
    // res created first, dropped last
    // s1 & s2 dropped before res
    // thus, both s1 and s2  don't live long enough to satsfiy 'a
    // b/c 'a must be as long as res's lifetime

    // let res;
    // let s1;
    // let s2;
    // {
    //     s1 = String::from("abcd");
    //     s2 = String::from("xyz");
    //     res = longest(s1.as_str(), s2.as_str());
    // }
    // assert_eq!(res, "abcd");

    // won't work (s2 does not live long enough to satisfy 'a lifetime constraint)
    // let s1 = String::from("abcd");
    // let res;
    // {
    //     let s2 = String::from("xyz");
    //     res = longest(s1.as_str(), s2.as_str());
    // }
    // assert_eq!(res, "abcd");

    // wont work (both s1 and s2 don't live long enough to satisfy 'a)

    // let res;
    // {
    //     let s1 = String::from("abcd");
    //     let s2 = String::from("xyz");
    //     res = longest(s1.as_str(), s2.as_str());
    // }
    // assert_eq!(res, "abcd");

    ////////////////////////////
    // lifetime elision rules //
    ////////////////////////////

    // 1. each param that is ref gets its own lifetime param
    // 2. if: exactly 1 input lifetime param,
    //    then: that lifetime is assigned to all output lifetime params
    // 3. if: multiple input lifetime params, but one is &self/&mut self (ie: method),
    //    then: that lifetime is is assigned to all output lifetime params

    // rule 2 examples:

    // fn first_word(s: &str) -> &str
    // becomes
    // fn first_word<'a>(s: &'a str) -> &a' str
    // and compiler satisfied

    // but:
    // fn longest(x: &str, y: &str) -> &str
    // becomes:
    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'??? str
    // compiler NOT satisfied

    //////////////////////////
    // lifetimes in methods //
    //////////////////////////

    // 'a guarantees that ImportantExcerpt cannot outlive reference to `part`
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // so this is okay...
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    assert_eq!(i.part, "Call me Ishmael");

    // rule 3 example:

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    i.announce_and_return_part("the train is late!");

    // method becomes:
    // fn announce_and_return_part<'a, 'b>(&'a self, assnouncement: &'b str) -> &'a str

    //////////////////////////////////////////
    // combining lifetimes and trait bounds //
    //////////////////////////////////////////

    fn longest_with_alert<'a, T>(x: &'a str, y: &'a str, alert: T) -> &'a str
    where
        T: Display,
    {
        println!("Alert! {}", alert);
        longest(x, y)
    }
    assert_eq!(longest_with_alert(s1.as_str(), s2.as_str(), "WHOA!"), s1)
}
