rust
#![feature(specialization)]
#![allow(incomplete_features)]

trait Trait {
    type AssocType;
}

struct Struct {}

impl Trait for Struct {
    default type AssocType = i32;
}

type AssocType = <Struct as Trait>::AssocType;

fn main() {
    dbg!(std::any::type_name::<AssocType>());
    let x: AssocType = 0;
}
