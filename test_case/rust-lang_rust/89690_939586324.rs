rust
#![allow(incomplete_features)]
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
pub enum A {
    A,
    B,
}

pub trait B<const __: A> {
    fn a(&self) {}
}

struct C;

impl B<{ A::A }> for C {}

impl B<{ A::B }> for C {}

fn no_ice() {
    B::<{ A::A }>::a(&C);
}
