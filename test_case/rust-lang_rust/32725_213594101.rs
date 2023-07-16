
use std::ops::DerefMut;
use std::fmt::Debug;

fn main() {
    let mut r: Box<Debug> = Box::new(1);
    let x = Box::deref_mut(&mut r);
    let x : &mut Debug = x;
}
