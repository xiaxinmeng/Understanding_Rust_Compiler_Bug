rust
#![deny(missing_copy_implementations)]

mod inner {
    pub struct Foo {
        pub field: i32
    }
}

pub fn foo() -> inner::Foo {
    inner::Foo { field: 42 }
}

fn main() {}
