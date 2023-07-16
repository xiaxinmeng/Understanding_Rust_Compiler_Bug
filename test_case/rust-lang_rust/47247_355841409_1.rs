
#![feature(try_from)]
use std::convert::TryFrom;

fn foo<T>(x: T) {}
fn main() {
    foo::<u32>(u32::try_from(3_i32).unwrap());
}
