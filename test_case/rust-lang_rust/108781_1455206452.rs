rust
#![feature(min_specialization)]

struct S<const N: usize>;
impl<const N: i32> Copy for S<N> {}
impl<const N: usize> Copy for S<N> {}
