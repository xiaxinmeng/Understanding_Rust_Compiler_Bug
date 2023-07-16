rust
#![feature(self_struct_ctor)]

enum Foo {}

impl Foo {
    fn bar() {
        Self();
    }
}
