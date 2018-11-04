extern crate oop;
use oop::adt_blog;
use oop::blog;
use oop::gui::{Button, Draw, Screen};
use oop::idiomatic_blog;

// add a new impl of Draw trait
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // some drawing code..
    }
}

fn main() {
    // gui: duck-typing trait objects

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    // blog: state pattern

    let mut post = blog::Post::new();

    post.add_text("I ate a salad for lunch today!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    // reverts post to draft state
    assert_eq!("", post.content());

    post.approve();
    // since post in draft state, this returns empty string
    assert_eq!("", post.content());

    post.request_review(); // puts post to pending review
    post.approve();
    assert_eq!("", post.content()); // 2 approvals required so still not published

    post.approve(); // 2nd approval: now we can read it!
    assert_eq!("I ate a salad for lunch today!", post.content());

    // adding test only in draft form....

    let mut second_post = blog::Post::new();

    second_post.add_text("foo");
    second_post.request_review();
    second_post.add_text("bar");
    second_post.approve();
    second_post.approve();
    second_post.add_text("baz");
    assert_eq!("foo", second_post.content());

    // idiomatic_blog: encoding states in type system

    let mut p = idiomatic_blog::Post::new();
    p.add_text("I ate a salad for lunch today!");
    let p = p.request_review();
    let p = p.reject();
    let p = p.request_review();
    let p = p.approve();
    let p = p.approve();
    assert_eq!("I ate a salad for lunch today!", p.content());

    // adt blog (for shits and giggles!)

    let mut x = adt_blog::Post::new();
    x.add_text("I ate a salad for lunch today!");
    let x = x.request_review();
    let x = x.reject();
    let x = x.request_review();
    let x = x.approve();
    let x = x.approve();
    assert_eq!(Some("I ate a salad for lunch today!"), x.content());
}
