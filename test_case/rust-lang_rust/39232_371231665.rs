rust
fn foo<T>(x: &T) {
    let y = x.clone(); // results in `Clone` being invoked on the `&T`, rather than *failing*
}
