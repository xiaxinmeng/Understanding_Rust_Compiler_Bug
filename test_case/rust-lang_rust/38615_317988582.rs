rust
fn apply_permutation<'a, T>(
    objects: &'a [T],
    permutation: &'a [usize],
) -> impl Iterator<Item = T> + 'a
where
    T: Copy,
{
    permutation.iter().map(move |&i| objects[i])
}
