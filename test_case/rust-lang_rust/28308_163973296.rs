 Rust
#![allow(unused_parens)]
macro_rules! my_assert1 { ($e:expr) => { if !$e { panic!() } } }
macro_rules! my_assert2 { ($e:expr) => { if !($e) { panic!() } } }
// Meaningless text goes here
// And some more meaningless text
fn main() {
   my_assert2!("test");
}
