rust
fn foo() {
    let mut xorcism = Xorcism::new("hello".as_bytes());
    xorcism.munge(&[1, 2, 3]);
    xorcism.munge(&[1, 2, 3]);
}
