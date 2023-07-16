rust
use std::ops::Add;

fn ref_add<'a, T>(a: &'a T, b: &'a T) -> T
where
    T: From<<&'a T as Add>::Output>,
    &'a T: Add,
{
    T::from(a + b)
}

fn ref_add_hrtb<T>(a: &T, b: &T) -> T
where
    T: for<'x> From<<&'x T as Add>::Output>,
    for<'x> &'x T: Add,
{
    T::from(a + b)
}

fn main() {
    let a = 2i32;
    let b = 3i32;
    assert_eq!(ref_add(&a, &b), 5i32); // pass
    assert_eq!(ref_add_hrtb(&a, &b), 5i32); // E0277
}
