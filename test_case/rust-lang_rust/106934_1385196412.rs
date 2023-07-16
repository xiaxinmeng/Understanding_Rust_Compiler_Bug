Rust
#[test]
#[cfg(not(bootstrap))]
fn offset_in_dst() {
    #[repr(C)]
    struct Foo {
        x: u8,
        y: u16,
        more: [u8],
    }

    const X_OFFSET: usize = offset_of!(Foo, x);
    const Y_OFFSET: usize = offset_of!(Foo, y);

    assert_eq!(X_OFFSET, 0);
    assert_eq!(Y_OFFSET, 2);
}
