rust
pub fn skip_manual() -> i16 {
    let mut sum = 0;
    for i in 0..LUT_LEN {
        sum += LUT[(i + 2) % LUT_LEN];
    }
    sum
}
