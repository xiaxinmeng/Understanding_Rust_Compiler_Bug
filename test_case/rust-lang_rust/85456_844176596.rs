rust
fn evil<T: Query>() -> fn() {
    T::query
}
