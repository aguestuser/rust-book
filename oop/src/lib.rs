pub mod adt_blog;
pub mod blog;
pub mod idiomatic_blog;

/*************************************
 * 17.1 examples (encapsulation)
 *************************************/

#[derive(PartialEq, Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64
    }
}

/***************************************************
 * 17.2 examples (duck-typing, open for extension)
 **************************************************/

// if we wrote a GUI library like this...

pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // note: this uses a trait object (`dyn Draw`)
        // this requires dynamic dispatch to find the `draw` method
        // (instead of static dispatch via monomporphic compiler optimizations)
        // which is necessary if we want screen to be able to call
        // `run` on a list of polymorphically defined `Draw` implementors
        // thus we get greater code flexibility at the cost
        // of a small runtime perf tradeoff
        pub components: Vec<Box<dyn Draw>>,
        // further note: we can only make a trait object out of
        // "object-safe" traits; criteria for which is that all methods must:
        // (1) not return `Self`
        // (2) not use generic type params
        // b/c doing either would require knowing concrete type info that
        // creating a type object requires that we forget
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // code to draw the button
        }
    }

    // a user of the library could extend it as demonstrated in main.rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut c = AveragedCollection {
            list: vec![],
            average: 0.0,
        };
        c.add(1);

        assert_eq!(
            c,
            AveragedCollection {
                list: vec![1],
                average: 1.0
            }
        )
    }

    #[test]
    fn test_remove() {
        let mut c = AveragedCollection {
            list: vec![1, 2, 3],
            average: 2.0,
        };
        let x = c.remove();

        assert_eq!(x, Some(3));
        assert_eq!(
            c,
            AveragedCollection {
                list: vec![1, 2],
                average: 1.5
            }
        )
    }

    #[test]
    fn test_extending_gui_lib() {}
}
