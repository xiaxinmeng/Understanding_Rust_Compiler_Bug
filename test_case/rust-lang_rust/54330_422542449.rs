rust
fn dyn_example() -> Box<dyn Send + 'static + 'static> {
    Box::new(())
}
