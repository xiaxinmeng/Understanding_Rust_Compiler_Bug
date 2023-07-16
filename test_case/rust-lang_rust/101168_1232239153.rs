rust
fn foo(x: bool, mut y: i32) -> i32 {
    if x {
        y = 42;
    }
    y
}
