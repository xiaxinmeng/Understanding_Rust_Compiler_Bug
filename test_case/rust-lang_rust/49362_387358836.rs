rust
#![feature(generic_associated_types)]

pub trait MyTrait {
    type InternalType<TypeA>;
}

pub struct GenericType<TypeA> {
    _dummy: TypeA,
}

pub struct MyStruct {}

impl MyTrait for MyStruct {
    type InternalType<TypeA> = GenericType<TypeA>;
}

// -----------------------------------------------------------------------------
fn main() {}
