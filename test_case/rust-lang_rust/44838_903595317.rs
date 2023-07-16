rust
fn test<'a, 'b>(a: SomeType<'a>, b: SomeType<'b>) {
    assert!(a == b);
}
