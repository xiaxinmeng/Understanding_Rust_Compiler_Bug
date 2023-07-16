 rust
use std::ops::{Add, DerefMut};

#[derive(Clone)]
struct Array<S>(S);

impl<A, S> Add for Array<S>
    where A: Clone + Add<Output=A>,
          S: DerefMut<Target=[A]>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        panic!()
    }
}

impl<A, S> Add<A> for Array<S>
    where A: Clone + Add<Output=A>,
          S: DerefMut<Target=[A]>,
{
    type Output = Self;
    fn add(self, rhs: A) -> Self {
        panic!()
    }
}

fn main() {
}

// error: overflow evaluating the requirement `<_ as core::ops::Deref>::Target`
