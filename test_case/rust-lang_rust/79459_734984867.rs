rust
#![feature(no_core)]
#![no_core]

pub trait AsQuery {
    type Query;
}

impl<T> AsQuery for T {
    type Query = usize;
}

pub trait SelectDsl {
    type Output;
}

impl<T> SelectDsl for T
where
    T: AsQuery,
    // comment this out to get the same error in rustc ...
    T::Query: SelectDsl,
{
    type Output = <T::Query as SelectDsl>::Output;
}

pub type Select<Source> = <Source as SelectDsl>::Output;
// ... or uncomment this to get the same error
//pub struct S(Select<usize>);
