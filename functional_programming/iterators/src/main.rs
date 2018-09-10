fn main() {}

#[test]
fn iter_functionality() {
    // mutable iterator w/ immutable references
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // mutable iterator with owned values
    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.into_iter();

    assert_eq!(v2_iter.next(), Some(1));
    assert_eq!(v2_iter.next(), Some(2));
    assert_eq!(v2_iter.next(), Some(3));
    assert_eq!(v2_iter.next(), None);

    // mutable iterator with mutable references
    let mut v3 = vec![1, 2, 3];
    let mut v3_iter = v3.iter_mut();

    assert_eq!(v3_iter.next(), Some(&mut 1));
    assert_eq!(v3_iter.next(), Some(&mut 2));
    assert_eq!(v3_iter.next(), Some(&mut 3));
    assert_eq!(v3_iter.next(), None);
}

// iterator adaptors

// may consume the iterator...

#[test]
fn iter_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // `sum` is example of a "consuming adapter" (takes ownership of iter)
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

// may produce new iterators...

#[test]
fn iter_map() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // we must use `collect` b/c `map` (like all iterator adaptors) is lazy
    // if we don't call `collect`, `map` will never get called
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

// may use a closure to capture var from environment...

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    // this fn takes ownership of both `shoes` and `my_size` (later via closure)
    shoes.into_iter().filter(|s| s.size == my_size).collect()
}

#[test]
fn filers_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    assert_eq!(
        shoes_in_my_size(shoes, 10),
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    )
}

// you can make your own Iterators!

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn count_iter() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn count_adapters() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(sum, 18)
}
