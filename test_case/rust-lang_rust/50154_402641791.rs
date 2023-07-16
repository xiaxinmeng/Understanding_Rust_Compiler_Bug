rust
fn baseline() {
    let data = i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64);
    let one_to_16 = i16x16::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let output = one_to_16.gt(i16x16::splat(2i16)).select(data + 1i16, data);
    // note: if the mask is often false for all lanes you could guard the select
    // behind an `if mask.any() { ... }`
    assert_eq!(
        output,
        i16x16::new(4, 8, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65)
    );
}
