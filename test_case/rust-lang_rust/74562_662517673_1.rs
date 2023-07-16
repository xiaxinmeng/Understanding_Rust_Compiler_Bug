rust
use std::mem;

/// Returns `true` if any byte in the word `v` is nonascii (>= 128). Snarfed
/// from `../str/mod.rs`, which does something similar for utf8 validation.
#[inline]
fn contains_nonascii(v: usize) -> bool {
    const NONASCII_MASK: usize = 0x80808080_80808080u64 as usize;
    (NONASCII_MASK & v) != 0
}

/// Optimized ASCII test that will use usize-at-a-time operations instead of
/// byte-at-a-time operations (when possible).
///
/// The algorithm we use here is pretty simple. If `s` is too short, we just
/// check each byte and be done with it. Otherwise:
///
/// - Read the first word with an unaligned load.
/// - Align the pointer, read subsequent words until end with aligned loads.
/// - If there's a tail, the last `usize` from `s` with an unaligned load.
///
/// If any of these loads produces something for which `contains_nonascii`
/// (above) returns true, then we know the answer is false.
#[inline]
pub fn is_ascii(s: &[u8]) -> bool {
    const USIZE_SIZE: usize = mem::size_of::<usize>();

    let len = s.len();
    let align_offset = s.as_ptr().align_offset(USIZE_SIZE);

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    if len < USIZE_SIZE || len < align_offset || USIZE_SIZE < mem::align_of::<usize>() {
        return s.iter().all(|b| b.is_ascii());
    }

    // We always read the first word unaligned, which means `align_offset` is
    // 0, we'd read the same value again for the aligned read.
    let offset_to_aligned = if align_offset == 0 { USIZE_SIZE } else { align_offset };

    let start = s.as_ptr();
    // SAFETY: We verify `len < USIZE_SIZE` above.
    let first_word = unsafe { (start as *const usize).read_unaligned() };

    if contains_nonascii(first_word) {
        return false;
    }
    // We checked this above, somewhat implicitly. Note that `offset_to_aligned`
    // is either `align_offset` or `USIZE_SIZE`, both of are explicitly checked
    // above.
    debug_assert!(offset_to_aligned <= len);

    // word_ptr is the (properly aligned) usize ptr we use to read the middle chunk of the slice.
    let mut word_ptr = unsafe { start.add(offset_to_aligned) as *const usize };

    // `byte_pos` is the byte index of `word_ptr`, used for loop end checks.
    let mut byte_pos = offset_to_aligned;

    // Paranoia check about alignment, since we're about to do a bunch of
    // unaligned loads. In practice this should be impossible barring a bug in
    // `align_offset` though.
    debug_assert_eq!((word_ptr as usize) % mem::align_of::<usize>(), 0);

    while byte_pos <= len - USIZE_SIZE {
        debug_assert!(
            // Sanity check that the read is in bounds
            (word_ptr as usize + USIZE_SIZE) <= (start.wrapping_add(len) as usize) &&
            // And that our assumptions about `byte_pos` hold.
            (word_ptr as usize) - (start as usize) == byte_pos
        );

        // Safety: We know `word_ptr` is properly aligned (because of
        // `align_offset`), and we know that we have enough bytes between `word_ptr` and the end
        let word = unsafe { word_ptr.read() };
        if contains_nonascii(word) {
            return false;
        }

        byte_pos += USIZE_SIZE;
        // SAFETY: We know that `byte_pos <= len - USIZE_SIZE`, which means that
        // after this `add`, `word_ptr` will be at most one-past-the-end.
        word_ptr = unsafe { word_ptr.add(1) };
    }

    // If we have anything left over, it should be at-most 1 usize worth of bytes,
    // which we check with a read_unaligned.
    if byte_pos == len {
        return true;
    }

    // Sanity check to ensure there really is only one `usize` left. This should
    // be guaranteed by our loop condition.
    debug_assert!(byte_pos < len && len - byte_pos < USIZE_SIZE);

    // SAFETY: This relies on `len >= USIZE_SIZE`, which we check at the start.
    let last_word = unsafe { (start.add(len - USIZE_SIZE) as *const usize).read_unaligned() };

    !contains_nonascii(last_word)
}

/// Optimized ASCII test that will use usize-at-a-time operations instead of
/// byte-at-a-time operations (when possible).
///
/// The algorithm we use here is pretty simple. If `s` is too short, we just
/// check each byte and be done with it. Otherwise:
///
/// - Read the first word with an unaligned load.
/// - Align the pointer, read subsequent words until end with aligned loads.
/// - If there's a tail, the last `usize` from `s` with an unaligned load.
///
/// If any of these loads produces something for which `contains_nonascii`
/// (above) returns true, then we know the answer is false.
#[inline]
pub fn is_ascii_new(s: &[u8]) -> bool {
    const USIZE_SIZE: usize = mem::size_of::<usize>();

    let len = s.len();
    let align_offset = s.as_ptr().align_offset(USIZE_SIZE);

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    if len < USIZE_SIZE || len < align_offset || USIZE_SIZE < mem::align_of::<usize>() {
        return s.iter().all(|b| b.is_ascii());
    }

    // We always read the first word unaligned, which means `align_offset` is
    // 0, we'd read the same value again for the aligned read.
    let offset_to_aligned = if align_offset == 0 { USIZE_SIZE } else { align_offset };

    let start = s.as_ptr();
    // SAFETY: We verify `len < USIZE_SIZE` above.
    let first_word = unsafe { (start as *const usize).read_unaligned() };

    if contains_nonascii(first_word) {
        return false;
    }
    // We checked this above, somewhat implicitly. Note that `offset_to_aligned`
    // is either `align_offset` or `USIZE_SIZE`, both of are explicitly checked
    // above.
    debug_assert!(offset_to_aligned <= len);

    // word_ptr is the (properly aligned) usize ptr we use to read the middle chunk of the slice.
    let mut word_ptr = unsafe { start.add(offset_to_aligned) as *const usize };

    // `byte_pos` is the byte index of `word_ptr`, used for loop end checks.
    let mut byte_pos = offset_to_aligned;

    // Paranoia check about alignment, since we're about to do a bunch of
    // unaligned loads. In practice this should be impossible barring a bug in
    // `align_offset` though.
    debug_assert_eq!((word_ptr as usize) % mem::align_of::<usize>(), 0);

    while byte_pos < len - USIZE_SIZE {
        debug_assert!(
            // Sanity check that the read is in bounds
            (word_ptr as usize + USIZE_SIZE) <= (start.wrapping_add(len) as usize) &&
            // And that our assumptions about `byte_pos` hold.
            (word_ptr as usize) - (start as usize) == byte_pos
        );

        // Safety: We know `word_ptr` is properly aligned (because of
        // `align_offset`), and we know that we have enough bytes between `word_ptr` and the end
        let word = unsafe { word_ptr.read() };
        if contains_nonascii(word) {
            return false;
        }

        byte_pos += USIZE_SIZE;
        // SAFETY: We know that `byte_pos <= len - USIZE_SIZE`, which means that
        // after this `add`, `word_ptr` will be at most one-past-the-end.
        word_ptr = unsafe { word_ptr.add(1) };
    }

    // Sanity check to ensure there really is only one `usize` left. This should
    // be guaranteed by our loop condition.
    debug_assert!(byte_pos <= len && len - byte_pos <= USIZE_SIZE);

    // SAFETY: This relies on `len >= USIZE_SIZE`, which we check at the start.
    let last_word = unsafe { (start.add(len - USIZE_SIZE) as *const usize).read_unaligned() };

    !contains_nonascii(last_word)
}
