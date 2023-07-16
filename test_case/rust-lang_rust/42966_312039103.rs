rust
fn should_equal<'a, 'b>(a: Option<&'a usize>, b: &'b usize) {
    let b = Some(b);
    assert_eq!(a, b);
    assert_eq!(b, a);
}
