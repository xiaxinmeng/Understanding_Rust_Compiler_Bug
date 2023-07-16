rust
fn read_string<'a>(str: *const str) -> &'a str {
    unsafe { &*str }
}
