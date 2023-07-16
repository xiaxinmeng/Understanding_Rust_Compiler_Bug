rust
fn bar() -> impl Generator + Send {
    || {
        let bar: Box<dyn Send + 'static> = Box::new(5);
        yield;
        drop(bar)
    }
}
