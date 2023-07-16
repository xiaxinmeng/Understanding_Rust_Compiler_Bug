rust
pub fn bar<A>() {
    let _ = foo(|| ());
}

pub fn foo<B: 'static>(_: B) -> (usize, usize, TypeId) {
    (size_of::<B>, align_of::<B>, TypeId::of::<B>())
}
