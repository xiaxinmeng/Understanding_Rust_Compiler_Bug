rust
fn scope<'env>(c: impl FnOnce(&Scope<'env>) + 'env) {
    let s = Scope::new();
    c(&s)
}
