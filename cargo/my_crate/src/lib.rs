//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// assert_eq!(6, my_crate::add_one(5))
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_one() {
        assert_eq!(add_one(5), 6);
    }
}
