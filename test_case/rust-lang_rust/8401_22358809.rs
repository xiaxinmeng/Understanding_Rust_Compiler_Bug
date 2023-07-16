 rust
#[crate_type = "lib"];
#[link(name = "foo", vers = "0.1")];

use std::cast;

trait A {}
struct B;
impl A for B {}

pub fn bar<T>(_: &A) {}

fn foo<T>(b: &A) {
    let b: &A = unsafe { cast::transmute(b) };
    bar::<T>(b)
}
