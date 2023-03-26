use adder;

mod common;
mod abd;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}