rust
fn foo<'long: 'short, 'short>() {
    fn assert_impl_borrow<T: Borrow<Q>, Q>() {}
    assert_impl_borrow::<(&'long str, i32), (&'short str, i32)>();
}
