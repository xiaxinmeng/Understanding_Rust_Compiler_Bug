rust
pub fn foo<T: 'static>(_: T) -> (usize, usize, TypeId) {
    (size_of::<T>, align_of::<T>, TypeId::of::<T>())
}
