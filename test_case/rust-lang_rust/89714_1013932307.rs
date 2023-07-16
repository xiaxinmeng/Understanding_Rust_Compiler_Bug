rust
pub fn rorate_left_with_reverse(s: &mut [u32], mid: usize) {
    assert!(mid <= s.len());

    let _ = s[..mid].reverse();
    let _ = s[mid..].reverse();
    let _ = s.reverse();
}
