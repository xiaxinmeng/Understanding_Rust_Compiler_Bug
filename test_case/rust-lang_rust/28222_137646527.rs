 rust
#![feature(fnbox)]
use std::boxed::FnBox;
type Callback = Box<FnBox(&mut usize, &usize)>;
fn get_cb() -> Callback {
    Box::new(|a: &mut _, b: &_| {})
}
fn main() { }
