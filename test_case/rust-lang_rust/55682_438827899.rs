rust
pub struct SomeStruct<T> { _inner: T }

impl SomeStruct<()> {
    pub fn some_fn(&self) {}
}

impl SomeStruct<usize> {
    pub fn some_fn(&self) {}
}
