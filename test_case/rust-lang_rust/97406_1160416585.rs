rust
fn opaque<F>(_: F) -> impl Iterator { b"".iter() }
fn assert_static<T: 'static>(_: T) {}

fn good_generic_fn<T>() {
    // proving `<OpaqueTy<type_of(async {})> as Iterator>::Item: 'static`
    // somehow requires `T: 'static`.
    assert_static(opaque(async {}).next());
    assert_static(opaque(|| {}).next());
    assert_static(opaque(opaque(async {}).next()).next());
}
