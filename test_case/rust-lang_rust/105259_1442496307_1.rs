rust
pub fn variant_d(input: &[(usize, usize, usize, usize)]) -> usize {
    input
        .iter()
        .filter(|(a, b, c, d)| *a <= *c && *d <= *b || *c <= *a && *b <= *d)
        .count()
}
