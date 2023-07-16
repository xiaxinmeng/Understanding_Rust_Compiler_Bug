
#![feature(overloaded_calls)]
extern crate core;
use core::ops::Fn;

struct Bad<T> {
    a: Vec<T>
}   

impl<T> Fn<T, T> for Bad<T> {
    fn call(&self, _: T) -> T {
        self.data.slice_from(0).slice_to(1);
    }   
}

fn main() {
    let mut b = Vec::with_capacity(1);
    let mut v: Bad<uint> = Bad{a: b};
    println!("{}", v(0, 0));
}
