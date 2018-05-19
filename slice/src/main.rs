fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let also_hello = &s[..5];
    
    let world = &s[6..11];
    let also_world = &[6..];

    let full_slice = &s[..];

    println!("first word: {}", first_word(&s[..]));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' { return &s[0..i]; }
    }

    &s[..]
}

