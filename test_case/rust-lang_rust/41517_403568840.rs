rust
mod some_module {
    pub trait Foo {
        fn foo(&self);
    }
}

fn use_foo(object: &some_module::Foo) {
    object.foo();
}
