 Rust
use std::ptr;
trait Foo { fn foo(&self) {} }
impl Foo for *const u8 {}
impl<T: Fn()> Foo for *const T {}

fn ambig() {
    ptr::null().foo();
    //~ ERROR unable to infer enough type information about `_`
    // commenting one of the impls out makes it work
}

fn inambig<T>() where *const T: Foo {
    ptr::null().foo();
    // this works (but is just as ambiguous as the previous example)
}

fn main() {}
