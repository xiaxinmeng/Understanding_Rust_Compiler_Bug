 rust
use std::ops::DerefMut;
use std::fmt::Debug;

fn main() {
    let mut r: Box<Debug> = Box::new(1);
    let x: &mut (Debug + 'static) = Box::deref_mut(&mut r);
}
