//! #Art
//!
//! A library for modeling artistic concepts

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// the primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combine two primary colors in euqal amounts to create a secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) => Some(SecondaryColor::Orange),
            (PrimaryColor::Red, PrimaryColor::Blue) => Some(SecondaryColor::Purple),
            (PrimaryColor::Red, PrimaryColor::Red) => None,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => Some(SecondaryColor::Green),
            (PrimaryColor::Yellow, PrimaryColor::Red) => Some(SecondaryColor::Orange),
            (PrimaryColor::Yellow, PrimaryColor::Yellow) => None,
            (PrimaryColor::Blue, PrimaryColor::Red) => Some(SecondaryColor::Purple),
            (PrimaryColor::Blue, PrimaryColor::Yellow) => Some(SecondaryColor::Green),
            (PrimaryColor::Blue, PrimaryColor::Blue) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
