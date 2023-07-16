rust
pub fn variant_a_2(input: &[(usize, usize, usize, usize)]) -> usize {
    input
        .iter()
        .filter(|(a, b, c, d)| *a <= *c && *d <= *b || *c <= *a && *b <= *d)
        .count()
}
