rust
#![feature(inherent_associated_types)]
struct S;
impl S { type P<T> = T }
fn main() { let _: S::P<()> = (); }
