 rust
#![feature(overloaded_calls)]

use std::{fmt, ops};

struct Shower<T> {
    x: T
}

impl<T: fmt::Show> ops::Fn<(), ()> for Shower<T> {
    fn call(&self, _args: ()) {
        println!("{}", self.x);
    }
}

fn make_shower<T>(x: T) -> Shower<T> {
    Shower { x: x }
}

pub fn main() {
    let show3 = make_shower(3i);
    show3();
}
