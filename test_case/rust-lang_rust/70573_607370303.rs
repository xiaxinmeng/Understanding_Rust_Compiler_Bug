rust
#[cold]
#[inline(never)]  // <-- probably implied by `#[cold]` but no one knows for sure...
fn assert_failed(index: usize, len: usize) -> ! {
    panic!("insertion index (is {}) should be <= len (is {})", index, len);
}

if index > len {
    assert_failed(index, len);
}
