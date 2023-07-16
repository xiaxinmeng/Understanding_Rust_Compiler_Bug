rust
pub fn variant_e(input: &[(usize, usize, usize, usize)]) -> usize {
    input
        .iter()
        .filter(|(a, b, c, d)| a.cmp(c) == d.cmp(b))
        .count()
}
