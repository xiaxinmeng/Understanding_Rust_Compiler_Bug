rust
fn is_ascii_align_to_unrolled(bytes: &[u8]) -> bool {
    // Not really clear if this should be testing size_of [usize; 2] or size_of usize still...
    if bytes.len() < core::mem::size_of::<[usize; 2]>() {
        return bytes.iter().all(|b| b.is_ascii());
    }
    // SAFETY: transmuting a sequence of `u8` to `[usize; 2]` is always fine
    let (head, body, tail) = unsafe { bytes.align_to::<[usize; 2]>() };
    head.iter().all(|b| b.is_ascii())
        && body.iter().all(|w| !contains_nonascii(w[0] | w[1]))
        && tail.iter().all(|b| b.is_ascii())
}
