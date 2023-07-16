
#![feature(repr_transparent)]

struct Struct;

trait Trait {
    type Type;
}

#[repr(transparent)]
struct NumberWithUnit<'a>(<&'a Struct as Trait>::Type) where &'a Struct: Trait;

fn main() {}
