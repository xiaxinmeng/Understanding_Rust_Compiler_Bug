
#![feature(generic_associated_types)]

trait Trait {
    type Associated<'a>;
}

impl Trait for () {
    type Associated<'a> = &'a ();
}

fn main() {}
