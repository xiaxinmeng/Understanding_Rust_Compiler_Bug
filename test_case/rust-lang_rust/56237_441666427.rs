rust
use std::ops::{Deref, DerefMut};

fn foo<P>(_value: <P as Deref>::Target)
where
    P: DerefMut,
    <P as Deref>::Target: Sized,
{}

fn main() {
    foo::<&mut u32>(2);
}
