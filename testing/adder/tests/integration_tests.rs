extern crate adder;
mod common;

// can run only tests in this file with `cargo test --test integration_test`

#[test]
fn it_adds_two() {
    assert_eq!(adder::add_2(2), 4);
}

#[test]
fn it_adds_fixtures() {
    let nums: common::AdderFixtures = common::setup(2, 2);
    assert_eq!(adder::add(nums.x, nums.y), 4);
}
