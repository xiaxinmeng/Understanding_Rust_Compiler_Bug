rust
#![deny(clippy::missing_assert_message)]

fn main() {
    assert_eq!(1, 2, "oops");
}
