rust
#![feature(associated_type_bounds)]
use core::ops::Add;

pub trait RefOpsBound<'a, 'b, F, T: 'a, U: 'b>: Add<T, Output = U> {}
// Bound the type, and also the `S` type.
pub trait OpsBounds<'a, 'b, F, G: AsRef<Self> + 'a + 'b, S: RefOpsBound<'a, 'b, F, G, G>>: 'a + 'b + Sized + Add<Self, Output = Self> {}

pub trait WithVar: Sized {
    // This should imply Var: Add<Var, Output = Var> and &'a Var: Add<Var, Output = Var>
    type Var: for<'a, 'b> OpsBounds<'a, 'b, Self, &'a Self::Var, &'b Self> + Sized;
}

fn thing<'a, 'b, F: WithVar + 'b>(a: &'a F::Var, b: F::Var) {
    a + b;
}
