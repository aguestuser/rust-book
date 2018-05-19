fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // instantiating

    let u1 = User {
        email: String::from("someone@example.com"), // we don't use a &str here b/c would require lifetime param
        username: String::from("someusername123"), // (to store reference to data owned by someone else)
        active: true,
        sign_in_count: 1,
    };

    // println!("user 1's email: {}", u1.email);

    // mutating

    let mut u2 = User {
        email: String::from("someone@foo.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // println!("user 2's email: {}", u2.email);
    u2.email = String::from("someone@bar.com");
    // println!("user 2's email: {}", u2.email);

    // factories

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let u3 = build_user(String::from("foo@bar.com"), String::from("foo"));

    // println!("user 3's email: {}", u3.email);

    // cloning struct fields

    let u4 = User {
        email: String::from("bar@baz.com"),
        username: String::from("bar"),
        ..u3
    };

    // println!("user 4's email: {}", u4.email);

    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    black;

    // unit-like struct
    struct UnitLike {}

    /********************
     * RECTANGLE EXAMPLE
     ********************/

    // naive version

    let w = 30;
    let h = 50;

    fn area_naive(width: u32, height: u32) -> u32 {
        width * height
    }

    println!(
        "The area of the naive rectangle is {} square pixels",
        area_naive(w, h)
    );

    // better version

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    fn area(r: &Rectangle) -> u32 {
        r.width * r.height
    }

    println!(
        "The area of the better rectangle is {} square pixels",
        area(&rect1),
    );
    println!("Better rectangle is: {:#?}", rect1);

    // even better version

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    println!(
        "The area of the best rectangle is {} square pixels",
        rect1.area()
    );

    // taking multiple params

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 60,
    };

    impl Rectangle {
        // note that multiple impls are possible!
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    println!(
        "Rectangle 2 can hold rectangle 1? {}",
        rect2.can_hold(&rect1)
    );

    println!(
        "Rectangle 3 can hold rectangle 1? {}",
        rect3.can_hold(&rect1)
    );

    // associated methods

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    println!("This square has area: {}", Rectangle::square(5).area());
}
