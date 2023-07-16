rust
fn foo<const C: usize>() where (C - 2) {
    let x = [u8; C - 2];
}
fn bar() { foo::<1>(); }
