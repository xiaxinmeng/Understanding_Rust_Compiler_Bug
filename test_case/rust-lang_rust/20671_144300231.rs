 rust
use std::ops::*;

trait Vector<S> where
    for<'a, 'b> &'a Self: Add<&'b Self>,
    for<'a> &'a Self: Mul<S>,
{
    fn test(&self) {}
}

fn test<S, V: Vector<S>>(v: V) { v.test() }
