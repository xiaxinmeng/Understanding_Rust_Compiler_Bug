rust
fn foo<T>(_: T) {}

fn bar<'a>(_: &'a i32) {
    let _: &'static _ = &foo::<&'a i32>;
}
