rust
fn test_windows_zero() {
    let v: &[i32] = &[0, 1];
    let mut iter = v.windows(0);

    assert_eq!(iter.next(), Some(&[][..]));
    assert_eq!(iter.next(), Some(&[][..]));
    assert_eq!(iter.next(), Some(&[][..])); // causes a panic here
    assert_eq!(iter.next(), None);
}
