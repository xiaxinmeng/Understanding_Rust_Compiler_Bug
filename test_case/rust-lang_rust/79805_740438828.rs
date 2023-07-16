Rust
fn reduce<V>(f: impl Fn(V, Self::Item) -> V) -> Option<V> where Self::Item: Into<V>,  { /*...*/ }
