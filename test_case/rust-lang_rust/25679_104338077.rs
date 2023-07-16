 rust
impl<T> Foo<T> {
    fn present<D: Device<Resources=T>>(&self, _dev: &D) { } 
}
