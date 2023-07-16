rust
#![feature(min_specialization)]
#![feature(rustc_attrs)]

use std::fmt::Display;

#[rustc_specialization_trait]
trait Special {}

trait Foo {
    fn foo(&self);
}

impl<T: Display> Foo for T {
    default fn foo(&self) {
        println!("foo: {self}");
    }
}

impl<T: Display + Special> Foo for T {
    fn foo(&self) {
        println!("special foo: {self}");
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    Foo::foo(&vec);
}
