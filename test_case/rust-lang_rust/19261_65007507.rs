 rust
fn staticify<T>(x: &T) -> &'static T {
    x.clone()
}
