 rust
#![feature(associated_types)]

trait Borrow {
    type Owned;

    fn borrow<'a>(&'a Borrow::Owned) -> &'a Self;
}

impl Borrow for int {
    //type Owned = int;

    fn borrow(_: &int) -> &int {
        unimplemented!();
    }
}

fn main() {}
