rust
fn foo<F: FnMut(), W: Deref<Target = F> + DerefMut>(mut f: W) {
    f()
}
