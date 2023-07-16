rust
// ./src/lib.rs:

pub struct Foo {
    a_value: u8
}

pub struct Bar {
    foo: Foo
}

impl Bar {
    pub fn use_my_foo(&self) -> u8 {
        // Forget to change this to self.foo.a_value
        self.foo.a_value()
    }
}
