Rust
fn lambda<T: Default>() -> T {
    if true && let Some(bar) = transform() {
        bar
    } else {
        T::default()
    }
}

fn transform<T>() -> Option<T> {
    None
}
