
fn foo(p: *const i64) -> i64 {
    ptr::load_unaligned(p)
}
