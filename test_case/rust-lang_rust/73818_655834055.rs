rust
#![feature(specialization)]

#[derive(PartialEq)]
enum Never {}

trait Foo {
    type Assoc: PartialEq;
}

impl<T> Foo for T {
    default type Assoc = Never;
}
