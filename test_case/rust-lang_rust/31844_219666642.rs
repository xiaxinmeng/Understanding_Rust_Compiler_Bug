 rust
#![feature(specialization)]
pub trait Foo {}
pub trait Bar: Foo {}
pub trait Baz: Foo {}

pub trait Trait {
    type Item;
}

struct Staff<T> { }

impl<T: Foo> Trait for Staff<T> {
    default type Item = i32;
}

impl<T: Foo + Bar> Trait for Staff<T> {
    type Item = i64;
}

impl<T: Foo + Baz> Trait for Staff<T> {
    type Item = f64;
}

fn main() {
    let _ = Staff { };
}
