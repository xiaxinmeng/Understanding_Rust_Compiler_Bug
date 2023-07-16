rust
/// Same as `array[..=usize::from(n)].rotate_right(1);`
pub fn rotate_right1_fastest(array: &mut [u8; 256], n: u8) {
    debug_assert!(n < 6);

    let n = usize::from(n);

    let b = array[n];
    array[5 - usize::from(n < 5)] = array[4];
    array[4 - usize::from(n < 4)] = array[3];
    array[3 - usize::from(n < 3)] = array[2];
    array[2 - usize::from(n < 2)] = array[1];
    array[1 - usize::from(n < 1)] = array[0];
    array[0] = b;
}
