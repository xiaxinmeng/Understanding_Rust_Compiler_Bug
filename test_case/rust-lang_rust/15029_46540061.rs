 rust
extern crate core; // warning: use of experimental crate

fn main() {
     core::iter::Repeat::new(1); // no warning
}
