rust
fn foo(x: Box<[i32]>) {
    box *x;
}