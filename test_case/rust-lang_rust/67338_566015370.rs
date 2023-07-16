rust
pub trait TestTrait {
    fn test(self: Box<Self>);
}

pub struct Wrapper<T> {
    inner: T,
}

impl<T: TestTrait> TestTrait for Wrapper<T> {
    fn test(self: Box<Self>) {
        self.inner.test()
    }
}
