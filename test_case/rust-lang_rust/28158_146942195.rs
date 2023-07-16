
#![feature(zero_one)]
use std::ops::{Deref, Sub, Add};
use std::num::Zero;

fn homura<T: Deref<Target=i32>>(_: T) { }

#[derive(Copy, Clone)]
pub struct StdCounter<T: Zero + Add<T, Output = T> + Sub<T, Output = T>> {
    pub value: T
}

fn main() {}
