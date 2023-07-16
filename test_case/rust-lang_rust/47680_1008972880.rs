Rust
fn get_default<'a, T: Default>(opt: &'a mut Option<T>) -> &'a mut T {
    match opt.as_mut() {
        Some(value) => value,
        None => opt.insert(T::default()),
    }
}
