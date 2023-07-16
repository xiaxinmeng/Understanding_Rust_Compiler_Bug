
fn bar() -> i8 {
    return 123;
}
fn baz() -> bool {
    128 > bar() //~ ERROR comparison is useless due to type limits
}
