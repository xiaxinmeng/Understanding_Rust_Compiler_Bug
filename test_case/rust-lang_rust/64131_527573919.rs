rust
fn into_sorted_vector(self) -> Vec<(K, V)> /* or Vec<T> */
where K: PartialOrd,
           V: PartialOrd,
{
    let mut v = self.base.into_iter().collect::<Vec<_>>();
    v.unstable_sort(); // this is interesting in that it's unstable but we're not actually adding instability here since the underlying map already has an unstable iteration order
    v
}
