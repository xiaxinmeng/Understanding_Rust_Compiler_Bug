 rust
#![feature(core, hash)]
use std::hash::{hash, SipHasher};

trait Foo {
    fn foo<T>(self) -> u64;
}
impl Foo for () {
    fn foo<T: 'static>(self) -> u64 {
        //    ^^^^^^^ this should cause an error, the bound is missing from the
        // method definition in the trait, and calls are checked against that.
        hash::<_, SipHasher>(unsafe{ &std::intrinsics::type_id::<&'static T>() })
    }
}
fn main() {
    println!("{}", ().foo::<&()>());
}
