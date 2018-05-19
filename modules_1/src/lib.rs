mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function();
            secret_function()
        }
        fn secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
    outermost::inside::inner_function(); // calls middle_secret_function, inside::secret_function()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
