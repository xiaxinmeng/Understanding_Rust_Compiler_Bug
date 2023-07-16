Rust
fn foo<F: FnMut()>(f: &F) {
    (*f)();
}
