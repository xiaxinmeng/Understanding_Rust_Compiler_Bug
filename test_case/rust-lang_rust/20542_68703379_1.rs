 rust
use std::ops::{Add};

pub struct Vector2<T> {
    pub elems: [T; 2],
}

impl <U, T: Add<U>> Add for Vector2<T> {
    pub fn add(self, rhs: &Vector2<U>) -> Vector2<Add<U>::Output> {
    }
}
