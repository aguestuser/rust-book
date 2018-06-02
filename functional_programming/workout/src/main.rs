use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, rand_num: u32) {
    let mut expensive_result = Cacher::new(|num| simulated_expensive_calculation(num));
    // let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today do {} pushups!", expensive_result.value(intensity));

        println!("Next do {} situps!", expensive_result.value(intensity));
    } else {
        if rand_num == 3 {
            println!("Take a break today! Drink some water!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<F, A, B>
where
    F: Fn(A) -> B,
    A: Hash + Eq + Copy,
    B: Copy,
{
    calculation: F,
    values: HashMap<A, B>,
}

impl<F, A, B> Cacher<F, A, B>
where
    F: Fn(A) -> B,
    A: Hash + Eq + Copy,
    B: Copy,
{
    fn new(calculation: F) -> Cacher<F, A, B> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    fn value(&mut self, arg: A) -> B {
        let val = self.values.entry(arg).or_insert((self.calculation)(arg));
        *val
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn cache_int_values() {
    let mut c = Cacher::new(|a| a);

    assert_eq!(c.value(1), 1);
    assert_eq!(c.value(2), 2);
    assert_eq!(c.value(1), 1);
}

#[test]
fn cache_string_values() {
    let mut c = Cacher::new(|a| a);

    assert_eq!(c.value("foo"), "foo");
    assert_eq!(c.value("bar"), "bar");
}

#[test]
fn cache_string_slice_lenghts() {
    let mut c = Cacher::new(|a: &str| a.len());

    assert_eq!(c.value(&"foo"), 3);
    assert_eq!(c.value(&"foos"), 4);
    assert_eq!(c.value(&"foo"), 3);
}
