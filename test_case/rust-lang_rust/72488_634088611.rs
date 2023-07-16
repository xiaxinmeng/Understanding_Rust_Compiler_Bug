rust
fn foo<T>(x: T) {
    assert!(x == 0);
}
// EDIT: note that you can do the following, maybe that's a better comparison?
fn foo<T>(x: T) {
    assert!(transmute::<T, [u8; size_of::<T>()]>(x).iter().all(|y| y == 0));
}
