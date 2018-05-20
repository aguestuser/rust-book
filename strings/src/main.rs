fn main() {
    //defn: The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

    /////////////////
    // new strings //
    /////////////////

    // new empty string
    let mut s = String::new();
    // pushing (borrowed) slices
    s.push_str("foo");
    s.push_str(" bar");

    // new string from literal
    let literal = "some text";
    let _s1 = literal.to_string();

    let _s2 = "some text".to_string();
    let _s3 = String::from("some text");

    // pushing chars
    let mut s4 = String::from("lo");
    s4.push('l');

    ///////////////////
    // concatenation //
    ///////////////////

    // with `+` operator
    let s5 = String::from("foo ");
    let s6 = String::from("bar");
    let _s7 = s5 + &s6; // s5 is moved to s7 and cannot be used anymore
                        // note: + delegates to fn w/ signature ~=
                        // fn add(self, s: &str) -> String
                        // this means that &s6: &String is deref coerced to &str
                        // upshot: we move s5,
                        // apend a borrowed copy of s6
                        // and return ownership of the result
                        // (more efficient than copying both strings)

    // with format! macro
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let _s8 = format!("{}-{}-{}", tic, tac, toe); // does not take ownership of any params

    /////////////////
    // no indexing //
    /////////////////

    let _s9 = String::from("foo");
    // let f = s9[0]; <- BOOM! (cannot index into strings)
    // WHY? mainly: UTF-8

    // rust respresents strings as vecotrs of bytes
    // but it also supports UTF-8 strings, so chars are represented w/ 2 bytes
    // -> indexing woudl return only 1 of 2 bytes necessary to represent char
    //    :. the compiler prevents this error

    // also: consider bytes, scalar values (chars), grapheme clusters
    // - in non-english languages, there can be multiple bytes per char (aka "scalar value")
    //   and multiple chars per grapheme cluster ("leter") b/c diacritics each get a char repr
    // - note: in the hindu example (below), we have 6 chars, but *18* bytes (ie: 3 chars per byte) why?
    // - :. REALLY no 1-to-1 mapping of byte to "letter"

    // slicing as workaround..
    let hello = "Здравствуйте";
    let s10 = &hello[0..4];
    assert!(s10 == "Зд");
    // let s10 = &hello[0..1]; <-- BOOM! invalid char boundary!

    ///////////////
    // iterating //
    ///////////////

    for c in "नमस्ते".chars() {
        println!("{}", c); // prints 6 chars (including diacritics)
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b); // prints 18 bytes
    }
}
