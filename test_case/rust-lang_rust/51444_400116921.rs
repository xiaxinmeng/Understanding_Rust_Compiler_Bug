
use std::fmt::Debug;

struct Foo { }

impl Foo {
    fn foo(&self) -> impl Debug {
        let _: &'static Foo = self;
        ()
    }
}

fn main() { }
