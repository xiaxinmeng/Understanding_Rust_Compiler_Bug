rust
fn adapt_foo_to_bar<F: Fn(Foo) + Default>(bar: Bar) {
    let f = F::default();
    f(bar.into());
}
