pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use a::series::of::nested_modules;
use TrafficLight::{Red, Yellow};

fn main() {
    nested_modules();
    let _red = Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;
}
