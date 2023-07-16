rust
use std::marker::{PhantomData};

pub trait Trait { }

impl Trait for i32 { }

#[derive(Copy, Clone)]
pub struct Struct<T> where T: Trait {
    _marker: PhantomData<*const T>,
}

#[derive(Copy, Clone)]
pub struct DoesntWork {
    a: Struct<i32>,
    b: Struct<i64>,
}

fn main() { }
