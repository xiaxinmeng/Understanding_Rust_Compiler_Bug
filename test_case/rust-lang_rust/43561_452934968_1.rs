rust
fn inferred_poly<'a, T: Foo + ?Sized + 'a>(t: &'a T) {
    T::foo();
}
