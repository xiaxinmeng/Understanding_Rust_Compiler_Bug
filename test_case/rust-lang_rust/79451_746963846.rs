rust
pub fn zip4<T, U, const N: usize>(lhs: [T; N], rhs: [U; N]) -> [(T, U); N] {
    let mut rhs = IntoIter::new(rhs);
    lhs.map(|lhs| (lhs, rhs.next().unwrap()))
}
