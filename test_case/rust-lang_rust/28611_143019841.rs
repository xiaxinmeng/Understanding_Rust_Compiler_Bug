 rust
fn foo(a: i64, b: i64) -> i64 {
    a.checked_mul(b).unwrap_or(0)
}
