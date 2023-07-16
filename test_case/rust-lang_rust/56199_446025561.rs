rust
#![feature(self_struct_ctor)]

struct Foo {}

impl Foo {
    fn bar() {
        Self()
    }
}
