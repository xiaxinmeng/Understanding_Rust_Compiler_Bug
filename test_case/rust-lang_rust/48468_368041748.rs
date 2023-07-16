rust
fn b<T>(items: &[T]) -> Box<dyn Iterator<Item=&T> + '_>;
