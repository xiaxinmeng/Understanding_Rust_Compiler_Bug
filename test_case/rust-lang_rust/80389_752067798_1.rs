rust
fn assert_impl_borrow<A, B>(_: A, _: B) where A: std::borrow::Borrow<B> {}

fn test<'a, 'b>(a: &'a (), b: &'b ()) {
    assert_impl_borrow::<&'a (), &'b ()>(a, b);
}
