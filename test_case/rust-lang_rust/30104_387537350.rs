rust
use std::ops::{Deref, DerefMut};

// Fails
fn smart_two_field(v: &mut Wrap<(i32, i32)>) {
    let _a = &mut v.0;
    let _b = &mut v.1;
}

fn smart_destructure(v: &mut Wrap<(i32, i32)>) {
    let (ref mut _head, ref mut _tail) = **v;
}

// Fails
fn box_two_field(v: &mut Box<(i32, i32)>) {
    let _a = &mut v.0;
    let _b = &mut v.1;
}

// Fails
fn box_destructure(v: &mut Box<(i32, i32)>) {
    let (ref mut _head, ref mut _tail) = **v;
}

// My own smart pointer
struct Wrap<T>(T);

impl<T> Deref for Wrap<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Wrap<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn main() {}
