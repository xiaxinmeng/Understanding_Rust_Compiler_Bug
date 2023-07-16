rust
#[derive(Debug)]
struct Foo(u8);

pub trait Access {
    fn field(&self) {}
}

impl<T> Access for T {}
