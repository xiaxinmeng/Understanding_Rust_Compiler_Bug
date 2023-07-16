rust
pub fn problematic(buf: &mut [u8], offsets: &[u8], mut idx: usize) {
    let offsets = &offsets[..=idx];
    for b in buf {
        *b = idx as u8;
        let tmp = std::cmp::min(offsets.len() - 1, idx);
        idx = idx.saturating_sub(usize::from(offsets[tmp]));
    }
}
