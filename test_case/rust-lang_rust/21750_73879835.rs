 rust
impl<'a> Traversal for i32 {
    type Item = &'a i32;
    fn foreach<F: Arg<<Self as Traversal>::Item>>(f: F) { }
}
