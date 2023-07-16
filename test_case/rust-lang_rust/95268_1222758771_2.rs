rust
pub trait B: for<'a> {
    Self: 'a,
    A<Z<'a> = Iter<'a, <Self as A>::Y<'a>>>
} {}
