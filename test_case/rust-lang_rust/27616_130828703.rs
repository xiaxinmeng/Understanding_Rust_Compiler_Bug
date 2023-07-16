 rust
fn foo(x: &'static mut i32) -> (&'static mut i32, &'static mut i32) {
    (x, x)
}
