use std::collections::HashMap;
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

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
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
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    assert_eq!(c.value(1), 1);
    assert_eq!(c.value(2), 2);
    assert_eq!(c.value(1), 1);
}
