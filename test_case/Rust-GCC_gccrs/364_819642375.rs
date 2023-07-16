rust
fn foo(c: i32) -> i32 {
    let b = if c > 0 {
        return 1;
    } else {
        c
    };
    b
}
