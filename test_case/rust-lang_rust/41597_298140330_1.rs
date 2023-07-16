rust
impl<T: MyBar> Bar for Bridge<T> {
    fn bar<F: Foo>(&self, f: F) {
        <Bridge<T> as BarBridge<F>>::bridge(self, f)
    }
}
