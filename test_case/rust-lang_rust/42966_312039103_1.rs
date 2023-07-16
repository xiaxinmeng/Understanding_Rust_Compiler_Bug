rust
fn should_equal<'a, 'b>(a: Option<&'a usize>, b: &'b usize) {
    let b = Some(b);
    assert!(a == b);
    assert!(b == a);
}
