rust
/// Test that the total size is less than the size required to store both the closure
/// and the value at the same time.
#[test]
fn test_size() {
    const SIZE: usize = 1024;

    type InitFn = impl FnOnce() -> [u8; SIZE];

    fn init_fn(data: [u8; SIZE]) -> InitFn {
        move || data
    }

    assert!(size_of::<LazyLock<[u8; SIZE], InitFn>>() < size_of::<Once>() + SIZE + size_of::<InitFn>());
}
