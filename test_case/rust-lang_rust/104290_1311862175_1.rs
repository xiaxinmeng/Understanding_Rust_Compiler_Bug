rust
pub fn with_hole(x: &mut [Option<u8>; 2]) {
    *x = [None, Some(2)]; // Two `mov`s
}

pub fn one_word(x: &mut [Option<u8>; 2]) {
    *x = [Some(1), Some(2)]; // One `mov`
}
