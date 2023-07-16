rust
fn box_me<T: Debug + Any>(t: T) -> Box<dyn Debug> {
    Box::new(t)
}
