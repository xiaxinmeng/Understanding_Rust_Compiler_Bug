rust
#![feature(generic_associated_types)]

trait Foo {
    type Assoc<T>;
}

impl Foo for () {
    type Assoc<T> = [T; 2*2];
}
