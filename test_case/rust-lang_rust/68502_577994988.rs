rust
fn foo() {
    struct NoCopy;
    let x = NoCopy;
    drop(x);
    let _ = x;
}
