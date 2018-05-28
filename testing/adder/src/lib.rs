pub fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn add_2(a: i32) -> i32 {
    a + 2
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    // bring code in outer module into scope
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("this will fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle {
            length: 2,
            width: 3,
        };

        let r2 = Rectangle {
            length: 1,
            width: 2,
        };
        assert!(r1.can_hold(&r2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let r1 = Rectangle {
            length: 2,
            width: 3,
        };

        let r2 = Rectangle {
            length: 1,
            width: 2,
        };
        assert!(!r2.can_hold(&r1));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_2(2));
    }

    #[test]
    fn it_tests_private_funcs() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn it_compares_two_rect_structs() {
        assert_eq!(
            Rectangle {
                length: 2,
                width: 2
            },
            Rectangle {
                length: 2,
                width: 2
            }
        )
    }

    #[test]
    fn greeting_contains_name() {
        let res = greet("Carol");
        let expected = "Carol";
        assert!(
            res.contains(expected),
            "Expecting greeting to contain {}, got: {}",
            expected,
            res
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn guess_greater_than_100_panics() {
        Guess::new(101);
    }
}
