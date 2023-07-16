rust
fn check(a: Option<&i32>, b: &i32) -> bool {
    a.eq(&Some(b))
}
