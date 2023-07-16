rust
// ./src/lib.rs:

pub trait Foo {
    a_value(&self) -> u8;
}

pub struct Bar {
    foo: Box<dyn Foo>
}

impl Bar {
    pub fn use_my_foo(&self) -> u8 {
        self.foo.a_value()
    }
}
