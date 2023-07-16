rust
fn f() -> ! {
    panic!("quux")
}
fn g() -> isize {
    match f() {
        true => 1,
        false => 0,
    }
}
