rust
fn _two_arg<'a, 'b>(a: &'a str, b: &'b str) -> &'b str {
    let _has_free = |_x: &str| b;
    _has_free(a)
}
