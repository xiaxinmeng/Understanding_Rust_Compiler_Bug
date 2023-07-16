rust
#![feature(nll)]
#![allow(unused_variables)]

pub trait TryTransform {
    fn try_transform<F>(self, f: F)
    where
        Self: Sized,
        F: FnOnce(Self);
}

impl<'a, T> TryTransform for &'a mut T {
    fn try_transform<F>(self, f: F)
    where
//        Self: Sized,
        F: FnOnce(Self),
    {
        let this: *mut T = self as *mut T;
        f(self);
    }
}

fn main() {
}
