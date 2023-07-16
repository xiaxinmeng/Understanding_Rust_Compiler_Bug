rust
fn foo(&self) -> impl Future<Output = T> { // Note: you never could return `Future<T>`...
    async { self.my_service.foo() } // ...and under the proposal you couldn't call `foo` outside of `async` either.
}
