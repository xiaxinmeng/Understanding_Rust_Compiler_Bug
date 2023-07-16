 Rust
use std::fmt;
use std::iter::IntoIterator;

fn ptr_cast<U: ?Sized, V: ?Sized>(u: *const U) -> *const V
{
   u as *const _
}

fn main() {
    let i0 = [5u32];
    let i1 = i0.into_iter();
    let i2 : &Iterator<Item=&u32> = &i1;
    let i3 = i2 as *const Iterator<Item=&u32>;
    let p: *const fmt::Display = ptr_cast::<_, fmt::Display>(i3);
}
