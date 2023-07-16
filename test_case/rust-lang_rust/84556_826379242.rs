rust
impl<T: Foo> const Bar for () {
    fn bar() {
        T::foo()
    } 
}
