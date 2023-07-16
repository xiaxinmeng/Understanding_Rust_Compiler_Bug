 rust
#![feature(specialization)]

pub trait NotSame {}

pub struct True;
pub struct False;

pub trait Sameness {
    type Same;
}

mod internal {
    pub trait PrivSameness {
        type Same;
    }
}

use internal::PrivSameness;

impl<A, B> Sameness for (A, B) {
    type Same = <Self as PrivSameness>::Same;
}

impl<A, B> PrivSameness for (A, B) {
    default type Same = False;
}
impl<A> PrivSameness for (A, A) {
    type Same = True;
}

impl<A, B> NotSame for (A, B) where (A, B): Sameness<Same=False> {}

fn not_same<A, B>() where (A, B): NotSame {}

fn main() {
    // would compile
    not_same::<i32, f32>();

    // would not compile
    // not_same::<i32, i32>();
}
