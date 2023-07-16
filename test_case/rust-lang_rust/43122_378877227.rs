rust
fn foo() -> impl Generator<Yield = Box<Debug + 'static>> {
    || {
        yield Box::new(123i32);
        yield Box::new("hello");
    }
}
