rust
pub fn foo(x: &[i32]) -> &[i32] {
    let (a, b) = x.split_at(x.len() / 2);
    &b[..a.len()]
}
