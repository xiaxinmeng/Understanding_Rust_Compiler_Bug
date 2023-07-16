 rust
fn foo<'a>(split: &[&'a [u8]]) -> (&'a [u8], &'a [u8]) {
    (
        unsafe { split.get_unchecked(0) },
        unsafe { split.get_unchecked(1) },
    )
}
