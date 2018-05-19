fn main() {
    // borrowing a reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // borrowing and mutating a reference
    let mut s2 = String::from("hello");
    change(&mut s2);
    // let r1 = &mut s2; <-- BOOM! (cannot borrow mutable reference more than once)
    // let

    let mut s3 = String::from("hello");

    let r1 = &s3; // no problem (immutable reference)
    let r2 = &s3; // no problem (immutable reference)
    let r3 = &mut s3; // BOOM! (cannot have mutable/immutable reference at same time)

    let hmm = no_dangle();


    // RULES:
    // - at any given time you can have *either* one mutable reference *or* N immutable references
    // - references must always be valid (ie: reference allocated memory)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn no_dangle() -> String { // NOT &String
    let s = String::from("foo");
    // &s <--- BOOM! (dangling reference not allowed; would return pointer to de-allocated memory)
    s // works
}
