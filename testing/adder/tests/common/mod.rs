pub struct AdderFixtures {
    pub x: i32,
    pub y: i32,
}

pub fn setup(x: i32, y: i32) -> AdderFixtures {
    AdderFixtures { x, y }
}
