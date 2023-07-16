rust
fn troublesome() {
    does_not_exist();
}

fn innocent_bystander() {
    assert_eq!("x".as_bytes(), &[]);
}
