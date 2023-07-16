rust
fn assert_impl_borrow<A, B>(_: A, _: B) where A: std::borrow::Borrow<B> {}

fn test<'b>(a: &'static (), b: &'b ()) {
    assert_impl_borrow::<&'static (), &'b ()>(a, b);
}
