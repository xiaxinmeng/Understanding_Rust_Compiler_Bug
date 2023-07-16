rust
fn foo<const C: usize>() {
    let x = [u8; C - 2];
}
fn bar() { foo::<1>(); }
