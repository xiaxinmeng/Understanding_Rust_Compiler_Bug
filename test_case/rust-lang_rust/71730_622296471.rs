rust
fn foo2() {
    let c = (1i32, 2i32);
    let &(x, y) = &c;
    // This works
}
