 Rust
use std::ops::{Mul};

pub trait M: Sized where f64: Mul<Self, Output=Self> {
}

pub trait M2: M
        where f64 : Mul<Self, Output=Self> // <- ADDED
{
    fn f(self, a: f64) -> Self {
        a * self
    }
}

fn main() {}
