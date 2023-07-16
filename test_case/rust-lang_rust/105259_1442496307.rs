rust
pub fn variant_c(input: &[(usize, usize, usize, usize)]) -> usize {
    input
        .iter()
        .filter(|(a, b, c, d)| ((d <= b) & (a <= c)) | ((c <= a) & (b <= d)))
        .count()
}
