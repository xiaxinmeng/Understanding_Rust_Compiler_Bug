rust
#[test]
fn test_pointer_alignment() {
    let bytes = ASCII.as_bytes();
    assert_ne!(intrinsic_alignment(bytes), 0);
    test(bytes);
}

fn intrinsic_alignment(bytes: &[u8]) -> usize {
    let usize_bytes = std::mem::size_of::<usize>();
    bytes.as_ptr().align_offset(usize_bytes)
}

fn test(bytes: &[u8]) {
    let usize_bytes = std::mem::size_of::<usize>();
    let align = bytes.as_ptr().align_offset(usize_bytes);
    let len = bytes.len();
    let mut index = 0;

    while index < len {
        let wrapping_sub = align.wrapping_sub(index) % usize_bytes;
        let align_offset = unsafe { bytes.as_ptr().add(index).align_offset(usize_bytes) };
        assert_eq!(wrapping_sub, align_offset);

        index += 1;
    }
}

static ASCII: &'static str = "\
hello this is a test \
hello this is a test \
hello this is a test \
hello this is a test \
hello this is a test \
hello this is a test \
";
