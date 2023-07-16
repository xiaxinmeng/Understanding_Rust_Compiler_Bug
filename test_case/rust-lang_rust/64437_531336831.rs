rust
fn inner<'a, T: 'a>(_: &'a ()) {}

fn outer<F>(_: F)
where
    F: Fn(&()),
{}

fn foo() {
    // Works:
    outer(|i| inner::<()>(i));
    // Does not work:
    outer(inner::<()>);
}
