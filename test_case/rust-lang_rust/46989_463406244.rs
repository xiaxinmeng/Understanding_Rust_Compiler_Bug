rust
impl<T, U> Foo for fn(T) -> U {
    type Assoc = (T, U);
    fn foo(self) {
        let x: T;
        let y: U;
        // ...
    }
}
