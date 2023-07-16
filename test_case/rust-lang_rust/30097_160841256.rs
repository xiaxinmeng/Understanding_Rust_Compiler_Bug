 rust
fn foo(x: i32) {
    assert!(x < 10);
    foo(x - 1);
}
