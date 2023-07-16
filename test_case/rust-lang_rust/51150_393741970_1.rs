rust
fn foo<'a, T: AsRef<[u8]> + ?Sized>(slice: &'a T) -> &'a [u8] {
    slice.as_ref()
}
