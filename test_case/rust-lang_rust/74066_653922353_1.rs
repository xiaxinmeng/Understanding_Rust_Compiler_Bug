rust
    fn is_ascii_slice_align_to(bytes: &[u8]) {
        fn contains_nonascii(v: usize) -> bool {
            const NONASCII_MASK: usize = 0x80808080_80808080u64 as usize;
            (NONASCII_MASK & v) != 0
        }
        let (head, body, tail) = unsafe { bytes.align_to::<usize>() };
        head.iter().all(|b| b.is_ascii()) &&
        body.iter().all(|w| !contains_nonascii(*w)) &&
        tail.iter().all(|b| b.is_ascii())
    }
